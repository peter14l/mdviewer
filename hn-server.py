import os
import re
from pathlib import Path
from typing import Optional
from mcp.server.fastmcp import FastMCP
from dotenv import load_dotenv
import httpx

load_dotenv(Path(__file__).parent / ".env")

HN_USERNAME = os.environ.get("HN_USERNAME", "")
HN_PASSWORD = os.environ.get("HN_PASSWORD", "")

mcp = FastMCP("hn-server")

FIREBASE = "https://hacker-news.firebaseio.com/v0"
ALGOLIA = "https://hn.algolia.com/api/v1"


def _login_session() -> httpx.Client | None:
    if not HN_USERNAME or not HN_PASSWORD:
        return None
    client = httpx.Client(cookies={}, follow_redirects=True)
    resp = client.post(
        "https://news.ycombinator.com/login",
        data={"acct": HN_USERNAME, "pw": HN_PASSWORD},
    )
    if "logout" not in resp.text:
        return None
    return client


@mcp.tool()
def submit_story(title: str, url: Optional[str] = None, text: Optional[str] = None) -> str:
    """Submit a story to Hacker News. Requires HN_USERNAME and HN_PASSWORD set."""
    client = _login_session()
    if not client:
        return "Error: HN login failed. Check HN_USERNAME/HN_PASSWORD."

    submit_page = client.get("https://news.ycombinator.com/submit")
    match = re.search(r'<input type="hidden" name="fnid" value="([^"]+)"', submit_page.text)
    if not match:
        return "Error: Could not get submission token."

    fnid = match.group(1)
    data = {"fnid": fnid, "title": title}
    if url:
        data["url"] = url
    if text:
        data["text"] = text

    resp = client.post("https://news.ycombinator.com/submit", data=data)
    client.close()

    if "Story submitted" in resp.text or "item?id=" in resp.text:
        match_id = re.search(r"item\?id=(\d+)", resp.text)
        story_id = match_id.group(1) if match_id else "unknown"
        return f"Story submitted successfully! ID: {story_id}"
    return "Story submission may have failed. Check HN credentials and try again."


@mcp.tool()
def get_top_stories(limit: int = 10) -> list:
    """Get current top stories from Hacker News."""
    resp = httpx.get(f"{FIREBASE}/topstories.json")
    ids = resp.json()[:limit]
    stories = []
    for sid in ids:
        s = httpx.get(f"{FIREBASE}/item/{sid}.json").json()
        if s and not s.get("deleted"):
            stories.append({
                "id": s["id"],
                "title": s.get("title", ""),
                "url": s.get("url", f"https://news.ycombinator.com/item?id={s['id']}"),
                "score": s.get("score", 0),
                "author": s.get("by", ""),
                "descendants": s.get("descendants", 0),
            })
    return stories


@mcp.tool()
def get_new_stories(limit: int = 10) -> list:
    """Get newest stories from Hacker News."""
    resp = httpx.get(f"{FIREBASE}/newstories.json")
    ids = resp.json()[:limit]
    stories = []
    for sid in ids:
        s = httpx.get(f"{FIREBASE}/item/{sid}.json").json()
        if s and not s.get("deleted"):
            stories.append({
                "id": s["id"],
                "title": s.get("title", ""),
                "url": s.get("url", f"https://news.ycombinator.com/item?id={s['id']}"),
                "score": s.get("score", 0),
                "author": s.get("by", ""),
            })
    return stories


@mcp.tool()
def search_stories(query: str, limit: int = 10) -> list:
    """Search Hacker News stories by query using Algolia."""
    resp = httpx.get(f"{ALGOLIA}/search", params={"query": query, "hitsPerPage": limit})
    data = resp.json()
    return [
        {
            "id": h["objectID"],
            "title": h.get("title", ""),
            "url": h.get("url", ""),
            "author": h.get("author", ""),
            "points": h.get("points", 0),
            "num_comments": h.get("num_comments", 0),
            "created_at": h.get("created_at", ""),
        }
        for h in data.get("hits", [])
    ]


@mcp.tool()
def get_story_details(story_id: int) -> dict:
    """Get full details of a specific HN story including top-level comments."""
    s = httpx.get(f"{FIREBASE}/item/{story_id}.json").json()
    if not s:
        return {"error": "Story not found"}
    comments = []
    for cid in (s.get("kids") or [])[:10]:
        c = httpx.get(f"{FIREBASE}/item/{cid}.json").json()
        if c and not c.get("deleted"):
            comments.append({
                "id": c["id"],
                "author": c.get("by", ""),
                "text": (c.get("text") or "")[:500],
                "score": c.get("score", 0),
            })
    return {
        "id": s["id"],
        "title": s.get("title", ""),
        "url": s.get("url", f"https://news.ycombinator.com/item?id={s['id']}"),
        "text": (s.get("text") or "")[:1000],
        "score": s.get("score", 0),
        "author": s.get("by", ""),
        "descendants": s.get("descendants", 0),
        "comments": comments,
    }


if __name__ == "__main__":
    mcp.run(transport="stdio")

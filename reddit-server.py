import os
from pathlib import Path
from typing import Optional
from mcp.server.fastmcp import FastMCP
from dotenv import load_dotenv
import praw

load_dotenv(Path(__file__).parent / ".env")

REDDIT_CLIENT_ID = os.environ.get("REDDIT_CLIENT_ID", "")
REDDIT_CLIENT_SECRET = os.environ.get("REDDIT_CLIENT_SECRET", "")
REDDIT_USERNAME = os.environ.get("REDDIT_USERNAME", "")
REDDIT_PASSWORD = os.environ.get("REDDIT_PASSWORD", "")

reddit = praw.Reddit(
    client_id=REDDIT_CLIENT_ID,
    client_secret=REDDIT_CLIENT_SECRET,
    username=REDDIT_USERNAME,
    password=REDDIT_PASSWORD,
    user_agent="opencode-mcp-server/v1.0",
)

mcp = FastMCP("reddit-server")


@mcp.tool()
def submit_post(
    subreddit: str,
    title: str,
    text: Optional[str] = None,
    url: Optional[str] = None,
    flair: Optional[str] = None,
) -> str:
    """Submit a text or link post to a subreddit."""
    sub = reddit.subreddit(subreddit)
    if url:
        post = sub.submit(title, url=url, flair_text=flair)
    else:
        post = sub.submit(title, selftext=text or "", flair_text=flair)
    return f"Created post: {post.url} (id: {post.id})"


@mcp.tool()
def submit_comment(post_url: str, text: str) -> str:
    """Reply to a Reddit post or comment with the given text."""
    submission = reddit.submission(url=post_url)
    comment = submission.reply(text)
    return f"Commented: https://reddit.com{comment.permalink} (id: {comment.id})"


@mcp.tool()
def search_reddit(query: str, subreddit: Optional[str] = None, sort: str = "relevance", limit: int = 10) -> list:
    """Search Reddit posts by query."""
    if subreddit:
        results = reddit.subreddit(subreddit).search(query, sort=sort, limit=limit)
    else:
        results = reddit.subreddit("all").search(query, sort=sort, limit=limit)
    return [
        {
            "id": p.id,
            "title": p.title,
            "url": p.url,
            "score": p.score,
            "subreddit": str(p.subreddit),
            "author": str(p.author),
            "created_utc": p.created_utc,
        }
        for p in results
    ]


@mcp.tool()
def get_hot_posts(subreddit: str = "all", limit: int = 10) -> list:
    """Get hot posts from a subreddit or the front page."""
    sub = reddit.subreddit(subreddit)
    return [
        {
            "id": p.id,
            "title": p.title,
            "url": p.url,
            "score": p.score,
            "author": str(p.author),
            "num_comments": p.num_comments,
        }
        for p in sub.hot(limit=limit)
    ]


@mcp.tool()
def get_top_posts(subreddit: str = "all", time_filter: str = "day", limit: int = 10) -> list:
    """Get top posts from a subreddit by time filter (hour, day, week, month, year, all)."""
    sub = reddit.subreddit(subreddit)
    return [
        {
            "id": p.id,
            "title": p.title,
            "url": p.url,
            "score": p.score,
            "author": str(p.author),
            "num_comments": p.num_comments,
        }
        for p in sub.top(time_filter=time_filter, limit=limit)
    ]


@mcp.tool()
def get_post_comments(post_url: str, limit: int = 10) -> list:
    """Get comments from a Reddit post."""
    submission = reddit.submission(url=post_url)
    submission.comments.replace_more(limit=0)
    return [
        {
            "id": c.id,
            "author": str(c.author),
            "body": c.body[:500],
            "score": c.score,
            "created_utc": c.created_utc,
        }
        for c in submission.comments[:limit]
    ]


if __name__ == "__main__":
    mcp.run(transport="stdio")

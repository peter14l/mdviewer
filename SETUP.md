# OpenCode MCP Servers — Setup Guide

Three MCP servers for automating posts on **Reddit**, **Hacker News**, and **dev.to**.
All files live in `D:\opencode-mcp-servers\`.

---

## Prerequisites

- **Python 3.12+** — installed at `C:\Program Files\Python312\python.exe`
- **Bun** — installed at `C:\Users\LOQ\.bun\bin\bun.exe`
- Required Python packages (already installed):

```bash
pip install mcp httpx praw beautifulsoup4 python-dotenv
```

---

## 1. Registering API Credentials

### Reddit — `https://www.reddit.com/prefs/apps`
1. Click **"create another app"**
2. Name: anything (e.g., `opencode-poster`)
3. Type: **script**
4. Redirect URI: `http://localhost:8080`
5. Click **"create app"**
6. Copy the **client ID** (under the app name) and **client secret**

### Hacker News
- **No API keys needed** for read operations (`get_top_stories`, `search_stories`, `get_new_stories`, `get_story_details`)
- For **posting** (`submit_story`): provide your HN login credentials (username + password)

### dev.to — `https://dev.to/settings/extensions`
1. Go to settings → Extensions (left sidebar)
2. Scroll to **"DEV Community API Keys"**
3. Add a description, click **"Generate API Key"**
4. Copy the generated key

---

## 2. Configuring Credentials

### `.env` file (for Reddit + HN)

Open `D:\opencode-mcp-servers\.env` and fill in:

```env
# Reddit API (from https://www.reddit.com/prefs/apps)
REDDIT_CLIENT_ID=your_client_id_here
REDDIT_CLIENT_SECRET=your_client_secret_here
REDDIT_USERNAME=your_reddit_username
REDDIT_PASSWORD=your_reddit_password

# Hacker News login (only needed for submitting stories)
HN_USERNAME=your_hn_username
HN_PASSWORD=your_hn_password
```

### `opencode.jsonc` (for dev.to)

Open `C:\Users\LOQ\.opencode\opencode.jsonc` and set:

```json
"devto": {
  "type": "local",
  "command": ["C:\\Users\\LOQ\\.bun\\bin\\bunx.exe", "@furkankoykiran/devto-mcp"],
  "enabled": true,
  "environment": {
    "DEVTO_API_KEY": "your_devto_api_key_here"
  }
}
```

---

## 3. Enabling Servers

In `C:\Users\LOQ\.opencode\opencode.jsonc`, set `"enabled": true` for each server:

| Server       | Default | Requires credentials to work |
|--------------|---------|------------------------------|
| `reddit`     | `false` | Yes — needs `.env`           |
| `hackernews` | `true`  | Read works without auth; `submit_story` needs `.env` |
| `devto`      | `false` | Yes — needs `DEVTO_API_KEY` in config |

---

## 4. Tools Reference

### Reddit Server (`reddit-server.py`)

| Tool | Description | Auth Required |
|------|-------------|---------------|
| `submit_post(subreddit, title, text?, url?, flair?)` | Submit a text or link post | Yes |
| `submit_comment(post_url, text)` | Reply to a post | Yes |
| `search_reddit(query, subreddit?, sort?, limit?)` | Search Reddit posts | Yes |
| `get_hot_posts(subreddit?, limit?)` | Get hot posts | Yes |
| `get_top_posts(subreddit?, time_filter?, limit?)` | Get top posts | Yes |
| `get_post_comments(post_url, limit?)` | Get comments on a post | Yes |

### Hacker News Server (`hn-server.py`)

| Tool | Description | Auth Required |
|------|-------------|---------------|
| `submit_story(title, url?, text?)` | Submit a story | Yes |
| `get_top_stories(limit?)` | Current top stories | No |
| `get_new_stories(limit?)` | Newest stories | No |
| `search_stories(query, limit?)` | Search via Algolia | No |
| `get_story_details(story_id)` | Story + top comments | No |

### dev.to Server (`@furkankoykiran/devto-mcp` via bunx)

| Tool | Description | Auth Required |
|------|-------------|---------------|
| `create_article(title, body_markdown, published?, tags?)` | Create article | Yes |
| `update_article(id, title?, body_markdown?, published?)` | Update article | Yes |
| `get_my_articles(status?, page?, per_page?)` | Own articles | Yes |
| `get_articles(tag?, username?, page?, per_page?)` | Browse articles | No |
| `get_article_by_id(id)` | Single article | No |
| `get_comments(a_id?)` | Article comments | No |
| `get_tags(page?, per_page?)` | List tags | No |
| `get_user_by_username(username)` | User profile | No |
| `get_organization(username)` | Org details | No |
| `get_followers(page?, per_page?)` | Your followers | Yes |
| `get_reading_list(page?, per_page?)` | Your reading list | Yes |
| `get_authenticated_user()` | Your profile | Yes |

---

## 5. Quick Verification

To verify a server is working, restart opencode and try a prompt like:

```
"Get the top 3 stories from Hacker News"
```

or if you have dev.to set up:

```
"Search dev.to for articles about rust"
```

---

## 6. File Structure

```
D:\opencode-mcp-servers\
├── .env                 # API credentials (Reddit + HN)
├── reddit-server.py     # Reddit MCP server
├── hn-server.py         # Hacker News MCP server
└── SETUP.md             # This file
```

Config reference: `C:\Users\LOQ\.opencode\opencode.jsonc`

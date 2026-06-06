from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
import uvicorn

app = FastAPI(title="QVX Terminal Dashboard")

# Mock data for demonstration
stats = {
    "compression_ratio": "156x",
    "total_data_processed": "1.2 TB",
    "memory_usage": "2.4 GB",
    "active_nodes": 5,
    "status": "Healthy"
}

templates = Jinja2Templates(directory="templates")

@app.get("/", response_class=HTMLResponse)
async def index(request: Request):
    return templates.TemplateResponse("index.html", {"request": request, "stats": stats})

@app.get("/api/stats")
async def get_stats():
    return stats

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8765)

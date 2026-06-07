# © 2026 Web Bite Labs | https://www.facebook.com/WebBiteLabs
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import hashlib
import time

app = FastAPI(title="QVX License Server")

MASTER_KEY = "QVX-MASTER-WEBBITLABS2026"

class LicenseCheck(BaseModel):
    key: str
    fingerprint: str

LICENSE_TIERS = {
    "TRIAL": {"features": [1, 2, 5, 6, 21], "days": 7},
    "STARTER": {"features": [1, 2, 3, 4, 5, 6, 7, 8, 11, 16, 17, 21], "days": 30},
    "PRO": {"features": list(range(1, 21)), "days": 365},
    "ENTERPRISE": {"features": list(range(1, 22)), "days": -1}
}

# Mock database
active_keys = {
    "QVX-TRIAL-2026FREE": {"tier": "TRIAL", "fingerprint": None, "revoked": False}
}

@app.post("/validate")
async def validate_license(check: LicenseCheck):
    if check.key not in active_keys:
        raise HTTPException(status_code=403, detail="INVALID_KEY")
    
    license_data = active_keys[check.key]
    if license_data["revoked"]:
        raise HTTPException(status_code=403, detail="LICENSE_REVOKED")
    
    if license_data["fingerprint"] is None:
        license_data["fingerprint"] = check.fingerprint
    elif license_data["fingerprint"] != check.fingerprint:
        raise HTTPException(status_code=403, detail="FINGERPRINT_MISMATCH")
    
    return {"status": "VALID", "tier": license_data["tier"], "features": LICENSE_TIERS[license_data["tier"]]["features"]}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=9000)

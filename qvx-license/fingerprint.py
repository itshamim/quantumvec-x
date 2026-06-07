# © 2026 Web Bite Labs | https://www.facebook.com/WebBiteLabs
import hashlib
import platform
import uuid

def get_hardware_fingerprint():
    # Combine: CPU + MAC + Node name
    info = platform.processor() + str(uuid.getnode()) + platform.node()
    return hashlib.sha256(info.encode()).hexdigest()

if __name__ == "__main__":
    print(f"Hardware Fingerprint: {get_hardware_fingerprint()}")

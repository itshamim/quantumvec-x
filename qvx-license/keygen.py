# © 2026 Web Bite Labs | https://www.facebook.com/WebBiteLabs
import hashlib
import random
import string

def generate_key(tier="PRO"):
    random_str = ''.join(random.choices(string.ascii_uppercase + string.digits, k=8))
    return f"QVX-{tier}-{random_str}"

if __name__ == "__main__":
    print(f"Generated Key: {generate_key()}")

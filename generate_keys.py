import hashlib
import secrets
import string

def generate_key():
    chars = string.ascii_uppercase + string.digits
    segments = [''.join(secrets.choice(chars) for _ in range(4)) for _ in range(3)]
    return f"SFPRO-{'-'.join(segments)}"

def hash_key(key):
    return hashlib.sha256(key.encode()).hexdigest()

count = 500  # generate 500 keys

keys = [generate_key() for _ in range(count)]
hashes = [hash_key(k) for k in keys]

# Save plain keys (for Gumroad delivery - KEEP THIS PRIVATE)
with open("keys.txt", "w") as f:
    f.write("\n".join(keys))

# Save hashes (goes into your binary)
with open("hashes.txt", "w") as f:
    f.write("\n".join(hashes))

print(f"Generated {count} keys")
print("keys.txt -> upload to Gumroad (KEEP PRIVATE)")
print("hashes.txt -> copy to keys/hashes.txt in your project")
"""Package the previously tested launcher without copying any runtime/user data."""
import hashlib
import json
import struct
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
source = ROOT / "incoming"
allowed = {"Corizon.exe", "LICENSE", "THIRD_PARTY_NOTICES.md", "README.txt", "build-info.json", "SHA256SUMS"}
actual = {str(p.relative_to(source)).replace("\\\\", "/") for p in source.rglob("*") if p.is_file()}
if actual != allowed or any(p.is_symlink() for p in source.rglob("*")):
    raise SystemExit(f"Unexpected distribution files: {sorted(actual ^ allowed)}")
exe = (source / "Corizon.exe").read_bytes()
expected = "f1d510bffd30a7f8f4cbe5e0228b1bdba2303969d52088448d6da8fc4a7d3e90"
assert hashlib.sha256(exe).hexdigest() == expected, "Executable differs from the tested build"
assert exe[:2] == b"MZ"
assert struct.unpack_from("<H", exe, struct.unpack_from("<I", exe, 0x3c)[0] + 4)[0] == 0x8664
info = json.loads((source / "build-info.json").read_text(encoding="utf-8-sig"))
assert info["commit"] == "4547dc321088abc9ec66aec4b751a353cdf64651"
assert info["version"] == "0.1.0-corizon-preview"
assert (source / "SHA256SUMS").read_text().strip() == expected + "  Corizon.exe"
out = ROOT / "dist"
out.mkdir(exist_ok=True)
archive = out / "Corizon-Launcher-Windows-x64.zip"
notes = (ROOT / "packaging/corizon-launcher-release.md").read_text(encoding="utf-8")
with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as z:
    for name in sorted(allowed):
        payload = (source / name).read_bytes()
        if name == "README.txt":
            payload = notes.encode("utf-8-sig")
        entry = zipfile.ZipInfo(name, (2026, 11, 8, 0, 0, 0))
        entry.compress_type = zipfile.ZIP_DEFLATED
        entry.external_attr = 0o100644 << 16
        z.writestr(entry, payload)
with zipfile.ZipFile(archive) as z:
    assert z.testzip() is None
    assert set(z.namelist()) == allowed
    assert hashlib.sha256(z.read("Corizon.exe")).hexdigest() == expected
digest = hashlib.sha256(archive.read_bytes()).hexdigest()
(out / "SHA256SUMS").write_text(digest + "  " + archive.name + "\n", encoding="ascii")
print("Verified clean archive:", archive.name)
print("Executable SHA256:", expected)
print("Archive SHA256:", digest)

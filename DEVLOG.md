### 5/9/2025 - 12:25 PM
**What I did**:
- Added `get_distros` command
- getting the distros successfully

**Problems**:
- output is garbage characters (encoding issue)
- before using `from_utf8_lossy`, switched to `from_utf8` (returns a Ok() of string)

**Next**:
- Making a menu for distros inf frontend

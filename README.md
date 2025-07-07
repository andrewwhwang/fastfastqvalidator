## Install:
```bash
git clone https://github.com/andrewwhwang/fastfastqvalidator
cd fastfastqvalidator
uv sync

# build a wheel file
maturin build -r
# or install python module directly into current venv
maturin develop -r
```

## Usage:
```python
from fastfastqvalidation import validate, FastqFormatError

fastq_filepath = "test.fastq"

try:
  validate("fastq_filepath")
  print("FASTQ file is valid.")
except FastqFormatError as e:
  print(f"FASTQ format error: {e}")
```

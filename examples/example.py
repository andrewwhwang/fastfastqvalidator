from fastfastqvalidation import validate, FastqFormatError
from time import perf_counter

if __name__ == "__main__":
    file_path = "/home/andrewwhwang/val_bench/test.fastq"
    start_time = perf_counter()
    # Example usage of the fq module
    try:
        # Validate a FASTQ file
        validate(file_path)
        print("FASTQ file is valid.")
    except FastqFormatError as e:
        print(f"FASTQ format error: {e}")
    print(perf_counter() - start_time)
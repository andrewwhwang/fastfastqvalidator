from fq import validate, FastqFormatError
from time import perf_counter

if __name__ == "__main__":

    start_time = perf_counter()
    # Example usage of the fq module
    try:
        # Validate a FASTQ file
        validate("/home/andrewwhwang/val_bench/test.fastq")
        print("FASTQ file is valid.")
    except FastqFormatError as e:
        print(f"FASTQ format error: {e}")
    print(perf_counter() - start_time)
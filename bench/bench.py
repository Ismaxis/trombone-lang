#!/usr/bin/env python3

import os
import sys
import subprocess
import time
import logging
import random

from pathlib import Path

_FACTORIAL_CALCULATION_FILE = "factorial_calculation"
_ARRAY_SORTING_FILE = "array_sorting"
_PRIME_NUMBER_GENERATION_FILE = "prime_number_generation"

_EXTENSION = ".cpp"

def setup_logging():
    """Setup logging configuration"""
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s',
        handlers=[
            logging.FileHandler('benchmark.log', mode='w')  # 'w' mode clears the file
        ]
    )
    return logging.getLogger(__name__)

def compile_program(compiler_path, source_file, output_file, logger):
    """Compile a program"""
    try:
        cmd = [compiler_path, '-O3', source_file, '-o', output_file]
        logger.info(f"Compiling: {' '.join(cmd)}")
        
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        
        if result.returncode == 0:
            logger.info(f"Compilation of {source_file} successful")
            return True
        else:
            logger.error(f"Compilation error for {source_file}: {result.stderr}")
            return False
            
    except subprocess.TimeoutExpired:
        logger.error(f"Compilation of {source_file} exceeded time limit")
        return False
    except Exception as e:
        logger.error(f"Exception during compilation of {source_file}: {e}")
        return False

def run_program_with_input(runtime_path, program_path, input_data, logger, timeout=10):
    """Run program with input data and measure execution time"""
    try:
        cmd = [runtime_path, program_path]
        logger.debug(f"Running: {' '.join(cmd)} with input: {input_data.strip()}")
        
        start_time = time.perf_counter()
        result = subprocess.run(
            cmd, 
            input=input_data, 
            capture_output=True, 
            text=True, 
            timeout=timeout
        )
        end_time = time.perf_counter()
        
        execution_time = end_time - start_time
        
        if result.returncode == 0:
            logger.info(f"Program executed in {execution_time*1000:.3f}ms")
            return execution_time, result.stdout.strip(), True
        else:
            logger.error(f"Execution error: {result.stderr}")
            return execution_time, result.stderr.strip(), False
            
    except subprocess.TimeoutExpired:
        logger.error(f"Program exceeded time limit ({timeout} seconds)")
        return timeout, "Timeout", False
    except Exception as e:
        logger.error(f"Exception during program execution: {e}")
        return 0, str(e), False

def validate_factorial_output(n, output, logger):
    """Validate factorial computation result"""
    try:
        result = int(output)
        expected = 1
        for i in range(1, n + 1):
            expected *= i
        
        if result == expected:
            logger.info(f"Factorial {n}! = {result} - correct")
            return True
        else:
            logger.error(f"Factorial {n}! incorrect: got {result}, expected {expected}")
            return False
    except ValueError:
        logger.error(f"Invalid factorial output: {output}")
        return False

def validate_sort_output(input_numbers, output, logger):
    """Validate sorting result"""
    try:
        output_numbers = list(map(int, output.split()))
        expected = sorted(input_numbers)
        
        if output_numbers == expected:
            logger.info(f"Sorting correct: {len(output_numbers)} elements sorted properly")
            return True
        else:
            differences = []
            min_len = min(len(output_numbers), len(expected))
            
            for i in range(min_len):
                if output_numbers[i] != expected[i]:
                    differences.append(f"index {i}: got {output_numbers[i]}, expected {expected[i]}")
            
            if len(output_numbers) != len(expected):
                differences.append(f"length mismatch: got {len(output_numbers)}, expected {len(expected)}")
            
            logger.error(f"Sorting incorrect. Differences: {'; '.join(differences[:5])}")
            if len(differences) > 5:
                logger.error(f"... and {len(differences) - 5} more differences")
            return False
    except ValueError:
        logger.error(f"Invalid sorting output: {output}")
        return False

def validate_sieve_output(n, output, logger):
    """Validate Sieve of Eratosthenes result"""
    try:
        if not output.strip():
            primes = []
        else:
            primes = list(map(int, output.split()))
        
        def is_prime(num):
            if num < 2:
                return False
            for i in range(2, int(num ** 0.5) + 1):
                if num % i == 0:
                    return False
            return True
        
        expected_primes = [i for i in range(2, n + 1) if is_prime(i)]
        
        if primes == expected_primes:
            logger.info(f"Sieve of Eratosthenes up to {n} correct: found {len(primes)} primes")
            return True
        else:
            logger.error(f"Sieve incorrect: got {len(primes)} primes, expected {len(expected_primes)}")
            return False
    except ValueError:
        logger.error(f"Invalid sieve output: {output}")
        return False

def test_factorial(runtime_path, program_path, logger):
    """Test factorial program"""
    logger.info("=== TESTING FACTORIAL ===")
    test_values = [1, 5, 10, 15, 20]
    results = []
    
    for n in test_values:
        logger.info(f"Testing factorial for n={n}")
        input_data = f"{n}\n"
        
        exec_time, output, success = run_program_with_input(runtime_path, program_path, input_data, logger)
        
        if success:
            is_valid = validate_factorial_output(n, output, logger)
            results.append((n, exec_time, is_valid))
        else:
            results.append((n, exec_time, False))
    
    return results

def test_sorting(runtime_path, program_path, logger):
    """Test sorting program"""
    logger.info("=== TESTING SORTING ===")
    test_cases = [
        [5, 3, 8, 1, 9],
        list(range(1000, 0, -1)),                                # 1000 elements in reverse order
        list(range(10000, 0, -2)),                               # 5000 even elements in reverse order
        [random.randint(1, 10000) for _ in range(100000)],       # 100000 random elements
        [random.randint(-10000, 10000) for _ in range(1000000)]  # 1 million random elements
    ]
    results = []
    
    for i, numbers in enumerate(test_cases, 1):
        logger.info(f"Testing sorting, case {i}: {len(numbers)} elements")
        n = len(numbers)
        input_data = f"{n}\n" + " ".join(map(str, numbers)) + "\n"
        
        exec_time, output, success = run_program_with_input(runtime_path, program_path, input_data, logger, timeout=30)
        
        if success:
            is_valid = validate_sort_output(numbers, output, logger)
            results.append((n, exec_time, is_valid))
        else:
            results.append((n, exec_time, False))
    
    return results

def test_sieve(runtime_path, program_path, logger):
    """Test Sieve program"""
    logger.info("=== TESTING SIEVE ===")
    test_values = [10, 100, 1000, 10000, 100000, 1000000]
    results = []
    
    for n in test_values:
        logger.info(f"Testing sieve for n={n}")
        input_data = f"{n}\n"
        
        exec_time, output, success = run_program_with_input(runtime_path, program_path, input_data, logger, timeout=60)
        
        if success:
            is_valid = validate_sieve_output(n, output, logger)
            results.append((n, exec_time, is_valid))
        else:
            results.append((n, exec_time, False))
    
    return results

def print_summary(program_name, results, logger):
    """Print results summary"""
    logger.info(f"\n=== SUMMARY FOR {program_name.upper()} ===")
    print(f"\n{program_name.upper()} - Test Results:")
    print("-" * 50)
    
    total_tests = len(results)
    passed_tests = sum(1 for _, _, valid in results if valid)
    
    for test_input, exec_time, is_valid in results:
        status = "✓ PASSED" if is_valid else "✗ FAILED"
        print(f"Input: {test_input:>8} | Time: {exec_time*1000:>8.3f}ms | {status}")
    
    print(f"\nPassed tests: {passed_tests}/{total_tests}")
    
    if results:
        avg_time = sum(exec_time for _, exec_time, _ in results) / len(results)
        min_time = min(exec_time for _, exec_time, _ in results)
        max_time = max(exec_time for _, exec_time, _ in results)
        
        print(f"Average time: {avg_time*1000:.3f}ms")
        print(f"Min time: {min_time*1000:.3f}ms")
        print(f"Max time: {max_time*1000:.3f}ms")

def main():
    if len(sys.argv) != 4:
        print("Usage: python script.py <compiler_path> <runtime_path> <tasks_directory>")
        sys.exit(1)
    
    compiler_path = sys.argv[1]
    runtime_path = sys.argv[2]
    tasks_dir = sys.argv[3]
    
    logger = setup_logging()
    logger.info("Starting benchmark script")
    logger.info(f"Compiler: {compiler_path}")
    logger.info(f"Runtime: {runtime_path}")
    logger.info(f"Tasks directory: {tasks_dir}")
    
    # Check if paths exist
    if not os.path.exists(compiler_path):
        logger.error(f"Compiler not found: {compiler_path}")
        sys.exit(1)
    
    if not os.path.exists(runtime_path):
        logger.error(f"Runtime not found: {runtime_path}")
        sys.exit(1)
        
    if not os.path.isdir(tasks_dir):
        logger.error(f"Tasks directory not found: {tasks_dir}")
        sys.exit(1)
    
    tasks_path = Path(tasks_dir)
    programs = [_FACTORIAL_CALCULATION_FILE, _ARRAY_SORTING_FILE, _PRIME_NUMBER_GENERATION_FILE]
    
    # Compile all programs
    logger.info("=== COMPILATION PHASE ===")
    compiled_programs = {}
    
    for program in programs:
        source_file = tasks_path / f"{program}{_EXTENSION}"
        output_file = tasks_path / f"{program}"
        
        if not source_file.exists():
            logger.error(f"Source file not found: {source_file}")
            continue
        
        if compile_program(compiler_path, str(source_file), str(output_file), logger):
            compiled_programs[program] = str(output_file)
    
    if not compiled_programs:
        logger.error("No programs compiled successfully")
        sys.exit(1)
    
    # Test programs
    logger.info("\n=== TESTING PHASE ===")
    
    all_results = {}
    
    # Test factorial
    if _FACTORIAL_CALCULATION_FILE in compiled_programs:
        factorial_results = test_factorial(runtime_path, compiled_programs[_FACTORIAL_CALCULATION_FILE], logger)
        all_results['Factorial'] = factorial_results
        print_summary('Factorial', factorial_results, logger)
    
    # Test sorting
    if _ARRAY_SORTING_FILE in compiled_programs:
        sort_results = test_sorting(runtime_path, compiled_programs[_ARRAY_SORTING_FILE], logger)
        all_results['Sorting'] = sort_results
        print_summary('Sorting', sort_results, logger)
    
    # Test sieve
    if _PRIME_NUMBER_GENERATION_FILE in compiled_programs:
        sieve_results = test_sieve(runtime_path, compiled_programs[_PRIME_NUMBER_GENERATION_FILE], logger)
        all_results['Sieve'] = sieve_results
        print_summary('Sieve', sieve_results, logger)
    
    # Overall summary
    logger.info("\n=== OVERALL SUMMARY ===")
    print("\n" + "="*60)
    print("OVERALL TESTING SUMMARY")
    print("="*60)
    
    total_programs = len(all_results)
    total_tests = sum(len(results) for results in all_results.values())
    total_passed = sum(sum(1 for _, _, valid in results if valid) for results in all_results.values())
    
    print(f"Programs tested: {total_programs}")
    print(f"Total tests: {total_tests}")
    print(f"Passed tests: {total_passed}")
    print(f"Success rate: {(total_passed/total_tests*100):.1f}%" if total_tests > 0 else "0%")
    
    logger.info("Testing completed")

if __name__ == "__main__":
    main()

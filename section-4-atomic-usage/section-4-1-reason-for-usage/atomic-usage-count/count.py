import os
import re
import sys
from collections import Counter

pattern_single = re.compile(
    r"\.(load|store|swap|update|compare_and_swap|fetch_add|fetch_sub|fetch_or|fetch_xor|fetch_nand|fetch_max|fetch_min)\(\s*[\s\S]*?"
    r"(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)(?:,[\s\S]*?)?\s*\)",
    re.DOTALL
)

pattern_double = re.compile(
    r"(compare_exchange(?:_weak)?)\s*\(\s*[\s\S]*?,\s*[\s\S]*?,\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)\s*,\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)(?:,[\s\S]*?)?\s*\)",
    re.DOTALL
)

struct_fetch_update = re.compile(
    r"\.\s*\w+\s*\.\s*(fetch_update)\(\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)\s*,\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)",
    re.DOTALL
)

global_fetch_update = re.compile(
    r"\b[A-Z_]+\s*\.\s*(fetch_update)\(\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)\s*,\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)",
    re.DOTALL
)

struct_field_single = re.compile(
    r"\.\w*[\.\w\(\)_\s]*\s*\.(load|store|swap|update|compare_and_swap|fetch_add|fetch_sub|fetch_or|fetch_xor|fetch_nand|fetch_max|fetch_min)"
    r"\(\s*[\s\S]*?(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)(?:,[\s\S]*?)?\s*\)",
    re.DOTALL
)

struct_field_double = re.compile(
    r"\.\w*[\.\w\(\)_\s]*\s*\.\s*(compare_exchange_weak|compare_exchange)\s*\(\s*[\s\S]*?,\s*[\s\S]*?,"
    r"\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)\s*,\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)(?:,[\s\S]*?)?\s*\)",
    re.DOTALL
)

global_single = re.compile(
    r"(?<![.\w])\b([A-Z_]+)\s*\.(load|store|swap|compare_and_swap|fetch_add|fetch_sub|fetch_or|fetch_xor|fetch_nand|fetch_max|fetch_min)"
    r"\(\s*.*?(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)(?:,[\s\S]*?)?\s*\)",
    re.DOTALL
)

global_double = re.compile(
    r"\b[A-Z_]+\s*\.\s*(compare_exchange_weak|compare_exchange)\s*\(\s*[\s\S]*?,\s*[\s\S]*?,\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)\s*,\s*(?:Ordering::)?(Relaxed|Acquire|Release|AcqRel|SeqCst)(?:,[\s\S]*?)?\s*\)",
    re.DOTALL
)


def find_atomic_ops(file_path, total_atomic_operations, ordering_counts):
    with open(file_path, 'r') as file:
        content = ''
        for line in file:
            if not line.strip().startswith("//"):
                content += line  # .replace('\n', '')

        atomic_signal = pattern_single.findall(content)
        atomic_double = pattern_double.findall(content)
        field_single = struct_field_single.findall(content)
        field_double = struct_field_double.findall(content)
        global_matches_single = global_single.findall(content)
        global_matches_double = global_double.findall(content)
        field_match_update = struct_fetch_update.findall(content)
        global_match_update = global_fetch_update.findall(content)
        atomic_operations_all = len(atomic_signal) + len(atomic_double) + len(field_match_update) + len(
            global_match_update)
        field_operations = len(field_single) + len(field_double) + len(field_match_update)
        global_operations = len(global_matches_single) + len(global_matches_double) + len(global_match_update)
        all_orderings = [order_signal for _, order_signal in atomic_signal] + [order_succ for _, order_succ, order_fal
                                                                               in atomic_double] + [
                            order8 for _, _, order8 in atomic_double] \
                        + [update_succ for _, update_succ, _
                           in field_match_update] + [
                            update_fal for _, _, update_fal in field_match_update] + [update_succ1 for
                                                                                      _, update_succ1, _
                                                                                      in global_match_update] + [
                            update_fal1 for _, _, update_fal1 in global_match_update]

        ordering_counts.update(all_orderings)
        return [field_operations, global_operations, atomic_operations_all]


# Traverse the source code directory
def walk_through_dir(dir_path):
    total_atomic_operations = 0
    field_atomic_operations = 0
    global_atomic_operations = 0
    ordering_counts = Counter()
    for root, dirs, files in os.walk(dir_path):
        # The use of memory ordering in the testing code is not calculated
        if root.endswith("tests") or root.endswith("examples") or "test" in root:
            print(root)
            continue

        for file in files:
            if file.endswith('.rs'):
                operation = find_atomic_ops(os.path.join(root, file), field_atomic_operations, ordering_counts)
                field_atomic_operations += operation[0]
                global_atomic_operations += operation[1]
                total_atomic_operations += operation[2]

    if total_atomic_operations > 0:
        struct_rate = field_atomic_operations / total_atomic_operations
    else:
        struct_rate = 0
    if global_atomic_operations > 0:
        global_rate = global_atomic_operations / total_atomic_operations
    else:
        global_rate = 0
    print("total_operations:", total_atomic_operations)
    print("field_atomic_operations:", field_atomic_operations)
    print("global_atomic_operations:", global_atomic_operations)
    print("other_atomic_operations:", total_atomic_operations-field_atomic_operations-global_atomic_operations)
    print(ordering_counts)
    if total_atomic_operations > 0:
        print("struct:", struct_rate)
        print("global:", global_rate)
        print("other:", 1 - struct_rate - global_rate)

    # Initialize a dictionary to store the percentages of each Ordering type
    ordering_ratios = {}

    # Initialize a variable to store the cumulative number of Acquire and Release
    acquire_release_count = 0

    for ordering, count in ordering_counts.items():
        if ordering in ["Acquire", "Release"]:
            # Count the number of Acquire and Release
            acquire_release_count += count
        else:
            # For other types of ordering, the percentage is calculated directly
            ratio = count / sum(ordering_counts.values()) if sum(ordering_counts.values()) > 0 else 0
            ordering_ratios[ordering] = ratio

    # Calculate the total percentage of Acquire and Release
    if sum(ordering_counts.values()) > 0:
        ordering_ratios["Acquire/Release"] = acquire_release_count / sum(ordering_counts.values())

    print(ordering_ratios)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <PROJECT_PATH>")
        sys.exit(1)
    project_dir = sys.argv[1]
    walk_through_dir(project_dir)

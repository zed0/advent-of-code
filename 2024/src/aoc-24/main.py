#!/usr/bin/env -S uv run
import sys
import operator
import math
import statistics
import itertools


def apply_operation(operation, inputs):
    if operation == "AND":
        return inputs[0] & inputs[1]
    elif operation == "OR":
        return inputs[0] | inputs[1]
    elif operation == "XOR":
        return inputs[0] ^ inputs[1]
    else:
        raise ValueError("Unknown operation:", operation)


def values_to_int(values, c):
    result = 0
    for name, value in values.items():
        if not name.startswith(c):
            continue
        if not value:
            continue
        result += 2 ** int(name[1:])
    return result


def calculate_values(values, gates):
    changed = True
    while changed:
        changed = False

        for output, (operation, inputs) in gates.items():
            if output in values:
                continue

            if inputs[0] not in values or inputs[1] not in values:
                continue

            input_values = (values[inputs[0]], values[inputs[1]])
            values[output] = apply_operation(operation, input_values)
            changed = True
    return values


def find_gates_used_by_output(output, gates):
    result = {}

    if output in gates:
        result[output] = gates[output]
        result.update(find_gates_used_by_output(gates[output][1][0], gates))
        result.update(find_gates_used_by_output(gates[output][1][1], gates))

    return result


def test_gates(output_bits, gates):
    for x_value in range(0, 4):
        for y_value in range(0, 4):
            values = {
                f"y{str(output_bits - 1).zfill(2)}": (y_value & 1) >> 0,
                f"y{str(output_bits).zfill(2)}": (y_value & 2) >> 1,
                f"x{str(output_bits - 1).zfill(2)}": (x_value & 1) >> 0,
                f"x{str(output_bits).zfill(2)}": (x_value & 2) >> 1,
            }
            for bit in range(output_bits - 1):
                values[f"x{str(bit).zfill(2)}"] = 0
                values[f"y{str(bit).zfill(2)}"] = 0

            calculated_values = calculate_values(values, gates)
            expected = (x_value + y_value) % 4

            output_1 = f"z{str(output_bits - 1).zfill(2)}"
            if (
                output_1 not in calculated_values
                or calculated_values[output_1] != expected & 1
            ):
                return False

            output_2 = f"z{str(output_bits).zfill(2)}"
            if (
                output_2 not in calculated_values
                or calculated_values[output_2] != (expected & 2) >> 1
            ):
                return False
    return True


def find_faults(values, gates):
    possible_output_bits = max(
        map(lambda k: int(k[1:]), filter(lambda k: k.startswith("z"), gates.keys()))
    )
    safe_gates = set()
    for output_bits in range(1, possible_output_bits):
        used_gates = find_gates_used_by_output(f"z{str(output_bits).zfill(2)}", gates)
        used_gates.update(
            find_gates_used_by_output(f"z{str(output_bits - 1).zfill(2)}", gates)
        )
        safe = test_gates(output_bits, used_gates)
        if safe:
            for gate in used_gates:
                safe_gates.add(gate)
        else:
            for suspicious_gate in used_gates:
                if suspicious_gate not in safe_gates:
                    print("suspicious gate:", suspicious_gate)

                    for replacement_gate in gates:
                        candidate_gates = used_gates.copy()
                        candidate_gates[suspicious_gate] = gates[replacement_gate]
                        candidate_gates.update(
                            find_gates_used_by_output(replacement_gate, gates)
                        )

                        success = test_gates(output_bits, candidate_gates)
                        if success:
                            print("\t", "found replacement:", replacement_gate)
                            return (suspicious_gate, replacement_gate)
                    print("\t", "no replacement found for", suspicious_gate)

    return None


print(f"input file: {sys.argv[1]}")

values = {}
gates = {}
with open(sys.argv[1]) as input_file:
    (input_lines, gate_lines) = input_file.read().split("\n\n")

    for line in input_lines.strip().split("\n"):
        (name, value) = line.split(": ")
        values[name] = int(value)

    for line in gate_lines.strip().split("\n"):
        (input_1, operation, input_2, _, output) = line.split()
        gates[output] = (operation, (input_1, input_2))

print("Part 1:")

calculated_values = calculate_values(values.copy(), gates)
result = values_to_int(calculated_values, "z")
print(result)

print("Part 2:")

fixed_gates = gates.copy()
bad_gates = []
while True:
    faults = find_faults(values.copy(), fixed_gates)
    if faults is None:
        break

    bad_gates += [faults[0], faults[1]]
    print("fixing", faults)
    (fixed_gates[faults[0]], fixed_gates[faults[1]]) = (
        fixed_gates[faults[1]],
        fixed_gates[faults[0]],
    )

print(",".join(sorted(bad_gates)))

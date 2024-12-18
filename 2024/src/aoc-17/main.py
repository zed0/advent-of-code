#!/usr/bin/env -S uv run
import sys


def combo(operand, registers):
    # Combo operands 0 through 3 represent literal values 0 through 3.
    if operand <= 3:
        return operand
    # Combo operand 4 represents the value of register A.
    elif operand == 4:
        return registers["A"]
    # Combo operand 5 represents the value of register B.
    elif operand == 5:
        return registers["B"]
    # Combo operand 6 represents the value of register C.
    elif operand == 6:
        return registers["C"]
    # Combo operand 7 is reserved and will not appear in valid programs.
    elif operand == 7:
        print("invalid operand")
        raise ValueError
    else:
        print("unknown operand: ", operand)


def run_program(program, registers, expected_output=None):
    instruction_pointer = 0
    output = []

    while True:
        if instruction_pointer >= len(program):
            if expected_output:
                matching = 0
                for idx in range(min(len(output), len(expected_output))):
                    if output[idx] != expected_output[idx]:
                        break
                    matching += 1
                return (output, matching)
            else:
                return output

        opcode = program[instruction_pointer]
        operand = program[instruction_pointer + 1]
        # print(instruction_pointer, opcode, operand)

        # The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
        if opcode == 0:
            # print("adv")
            registers["A"] = int(registers["A"] / (2 ** combo(operand, registers)))
            instruction_pointer += 2

        # The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
        elif opcode == 1:
            # print("bxl")
            registers["B"] = registers["B"] ^ operand
            instruction_pointer += 2

        # The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
        elif opcode == 2:
            # print("bst")
            registers["B"] = combo(operand, registers) % 8
            instruction_pointer += 2

        # The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
        elif opcode == 3:
            # print("jnz")
            if registers["A"] == 0:
                # print("not jumping")
                instruction_pointer += 2
            else:
                # print("jumping: ", operand)
                instruction_pointer = operand

        # The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
        elif opcode == 4:
            # print("bxc")
            registers["B"] = registers["B"] ^ registers["C"]
            instruction_pointer += 2

        # The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
        elif opcode == 5:
            # print("out")
            out = combo(operand, registers) % 8
            # if expected_output:
            #     if (
            #         len(output) >= len(expected_output)
            #         or expected_output[len(output)] != out
            #     ):
            #         return (output, len(output))

            output.append(out)
            instruction_pointer += 2

        # The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
        elif opcode == 6:
            # print("bdv")
            registers["B"] = int(registers["A"] / (2 ** combo(operand, registers)))
            instruction_pointer += 2

        # The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
        elif opcode == 7:
            # print("cdv")
            registers["C"] = int(registers["A"] / (2 ** combo(operand, registers)))
            instruction_pointer += 2


def run_program_with_modified_a_register(
    program, registers, register_a_octal, expected_output
):
    modified_registers = registers.copy()
    register_a_decimal = int(
        "".join(map(str, register_a_octal)),
        8,
    )
    modified_registers["A"] = register_a_decimal
    return run_program(program, modified_registers, expected_output)


print(f"input file: {sys.argv[1]}")

print("Part 1:")

registers = {}
program = []

with open(sys.argv[1]) as input_file:
    (register_lines, program_line) = input_file.read().strip().split("\n\n")
    for register_line in register_lines.split("\n"):
        (name, value) = register_line.split(": ")
        registers[name.split(" ")[1]] = int(value)

    program = list(map(int, (program_line.split(": ")[1].split(","))))

output = run_program(program, registers.copy())
print(",".join(map(str, output)))

print("Part 2:")

# [2, 4, 1, 2, 7, 5, 0, 3, 1, 7, 4, 1, 5, 5, 3, 0]

#  0: bst A => B = A % 8
#  2: bxl 2 => B = B ^ 2
#  4: cdv B => C = A / (2 ** B)
#  6: adv 3 => A = A / 8
#  8: bxl 7 => B = B ^ 7
# 10: bxc 1 => B = B ^ C
# 12: out B => output(B % 8)
# 14: jnz 0 => if A == 0: goto 0

checked = set()
to_check = [([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0, 0)]
valid = []
while len(to_check):
    (previous_register, digit, previous_match) = to_check.pop(0)
    checked.add((tuple(previous_register), digit))
    found = False
    for n in range(8):
        new_register = previous_register.copy()
        new_register[len(new_register) - 1 - digit] = n
        (output, match_len) = run_program_with_modified_a_register(
            program, registers, new_register, program.copy()
        )

        if match_len == len(program):
            # print("found valid input:", new_register, output)
            # print(int("".join(map(str, new_register)), 8))
            valid.append(int("".join(map(str, new_register)), 8))
            break
        else:
            if (
                (tuple(new_register), digit + 1) not in checked
                and digit < len(new_register)
                and match_len >= digit - 2
            ):
                to_check.append((new_register, digit + 1, match_len))

    to_check.sort(key=lambda x: (-x[2], x[1], x[0]))

print(min(valid))

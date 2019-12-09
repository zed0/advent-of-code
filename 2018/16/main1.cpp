#include <iostream>
#include <map>
#include <vector>
#include <regex>

typedef std::vector<int> registers;
typedef std::function<void(int, int, int, int, registers&)> operation;

struct instruction
{
	int opcode;
	int a;
	int b;
	int reg_out;

	void print()
	{
		std::cout << opcode << ", " << a << ", " << b << ", " << reg_out << std::endl;
	}
};

struct test_case
{
	instruction ins;
	registers registers_before;
	registers registers_after;

	void print()
	{
		for(const auto& reg : registers_before)
		{
			std::cout << reg;
		}
		std::cout << std::endl;

		ins.print();

		for(const auto& reg : registers_after)
		{
			std::cout << reg;
		}
		std::cout << std::endl;
	}

	bool test(operation op)
	{
		auto before = registers_before;
		op(ins.opcode, ins.a, ins.b, ins.reg_out, before);
		return before == registers_after;
	}

	static test_case read()
	{
		test_case tc;

		std::string line;
		std::getline(std::cin, line);
		if(line == "")
			throw std::runtime_error("test");

		std::smatch m;
		std::regex before_regex(R"regex(Before:\s+\[(\d+), (\d+), (\d+), (\d+)\])regex");
		std::regex_match(line, m, before_regex);
		tc.registers_before.push_back(std::stoi(m[1]));
		tc.registers_before.push_back(std::stoi(m[2]));
		tc.registers_before.push_back(std::stoi(m[3]));
		tc.registers_before.push_back(std::stoi(m[4]));

		instruction current_instruction;
		std::getline(std::cin, line);
		std::regex instruction_regex(R"regex((\d+) (\d+) (\d+) (\d+))regex");
		std::regex_match(line, m, instruction_regex);
		tc.ins.opcode = std::stoi(m[1]);
		tc.ins.a = std::stoi(m[2]);
		tc.ins.b = std::stoi(m[3]);
		tc.ins.reg_out = std::stoi(m[4]);

		std::getline(std::cin, line);
		std::regex after_regex(R"regex(After:\s+\[(\d+), (\d+), (\d+), (\d+)\])regex");
		std::regex_match(line, m, after_regex);
		tc.registers_after.push_back(std::stoi(m[1]));
		tc.registers_after.push_back(std::stoi(m[2]));
		tc.registers_after.push_back(std::stoi(m[3]));
		tc.registers_after.push_back(std::stoi(m[4]));

		return tc;
	}
};

//typedef std::function<void(int, int, int, int, registers&)> operation;
int main()
{
	std::map<std::string, operation> operations;
	operations["addr"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] + regs[b];};
	operations["addi"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] + b;};
	operations["mulr"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] * regs[b];};
	operations["muli"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] * b;};
	operations["banr"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] & regs[b];};
	operations["bani"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] & b;};
	operations["borr"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] | regs[b];};
	operations["bori"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] | b;};
	operations["setr"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a];};
	operations["seti"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = a;};
	operations["gtir"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = a > regs[b];};
	operations["gtri"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] > b;};
	operations["gtrr"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] > regs[b];};
	operations["eqir"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = a == regs[b];};
	operations["eqri"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] == b;};
	operations["eqrr"] = [](int opcode, int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] == regs[b];};

	int working_samples = 0;

	while(true)
	{
		test_case tc;
		try
		{
			tc = test_case::read();
			//tc.print();
			int passed = 0;
			for(const auto& operation : operations)
			{
				passed += tc.test(operation.second);
				if(passed >= 3)
				{
					working_samples++;
					break;
				}
			}
		}
		catch(const std::runtime_error& e)
		{
			break;
		}

		std::string line;
		if(!std::getline(std::cin, line))
			break;
	}

	std::cout << working_samples << std::endl;

	return 0;
}

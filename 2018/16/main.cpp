#include <iostream>
#include <map>
#include <vector>
#include <regex>
#include <sstream>

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

	static instruction read()
	{
		instruction ins;
		std::string line;
		if(!std::getline(std::cin, line))
			throw std::runtime_error("test");
		std::smatch m;
		std::regex instruction_regex(R"regex((\d+) (\d+) (\d+) (\d+))regex");
		std::regex_match(line, m, instruction_regex);
		ins.opcode = std::stoi(m[1]);
		ins.a = std::stoi(m[2]);
		ins.b = std::stoi(m[3]);
		ins.reg_out = std::stoi(m[4]);
		return ins;
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

	bool test(operation op) const
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

		tc.ins = instruction::read();

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

	std::string line;
	std::vector<test_case> test_cases;
	while(true)
	{
		test_case tc;
		try
		{
			tc = test_case::read();
			//tc.print();
			test_cases.push_back(tc);
		}
		catch(const std::runtime_error& e)
		{
			break;
		}

		if(!std::getline(std::cin, line))
			break;
	}

	std::getline(std::cin, line);

	std::vector<instruction> instructions;
	while(true)
	{
		instruction ins;
		try
		{
			ins = instruction::read();
			//ins.print();
			instructions.push_back(ins);
		}
		catch(const std::runtime_error& e)
		{
			break;
		}
	}


	auto test_operations = operations;

	std::map<int, std::string> op_numbers;
	while(test_operations.size())
	{
		for(const auto tc : test_cases)
		{
			if(op_numbers.count(tc.ins.opcode))
				continue;

			int passed = 0;
			std::string last_passed;
			for(const auto& operation : test_operations)
			{
				bool test = tc.test(operation.second);
				passed += test;
				if(test)
					last_passed = operation.first;
			}
			if(passed == 1)
			{
				if(op_numbers.count(tc.ins.opcode))
				{
					std::stringstream out_stream;
					out_stream << "duplicate entry: " << tc.ins.opcode << ": " << last_passed << " vs " << op_numbers[tc.ins.opcode];
					throw std::runtime_error(out_stream.str());
				}
				op_numbers[tc.ins.opcode] = last_passed;
				test_operations.erase(last_passed);
			}
		}
	}

	registers real_reg{0, 0, 0, 0};
	for(const auto& ins : instructions)
	{
		operations[op_numbers[ins.opcode]](ins.opcode, ins.a, ins.b, ins.reg_out, real_reg);
	}

	for(const auto& reg : real_reg)
	{
		std::cout << reg << ", ";
	}
	std::cout << std::endl;

	return 0;
}

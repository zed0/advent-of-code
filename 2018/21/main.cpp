#include <iostream>
#include <map>
#include <vector>
#include <regex>
#include <sstream>

typedef std::vector<int> registers;
typedef std::function<void(int, int, int, registers&)> operation;

struct instruction
{
	std::string op;
	int a;
	int b;
	int reg_out;

	void print() const
	{
		std::cout << op << "\t" << a << "\t" << b << "\t" << reg_out;
	}

	static instruction read()
	{
		instruction ins;
		std::string line;
		if(!std::getline(std::cin, line))
			throw std::runtime_error("test");
		std::smatch m;
		std::regex instruction_regex(R"regex((\w+) (\d+) (\d+) (\d+))regex");
		std::regex_match(line, m, instruction_regex);
		ins.op = m[1];
		ins.a = std::stoi(m[2]);
		ins.b = std::stoi(m[3]);
		ins.reg_out = std::stoi(m[4]);
		return ins;
	}
};

void print_registers(std::vector<int> registers)
{
	std::cout << "[";
	for(const auto& reg : registers)
	{
		std::cout << reg << ",\t";
	}
	std::cout << "]";
}

struct state
{
	state(int64_t tick_, registers reg_) :
		tick(tick_),
		reg(reg_)
	{
	}

	int64_t tick;
	registers reg;
};

std::vector<int32_t> vector_minus(const std::vector<int32_t>& a, const std::vector<int32_t>& b)
{
	std::vector<int32_t> result;
	for(int i=0; i<a.size(); ++i)
	{
		result.push_back(a[i] - b[i]);
	}
	return result;
}

std::vector<int32_t> vector_plus(const std::vector<int32_t>& a, const std::vector<int32_t>& b)
{
	std::vector<int32_t> result;
	for(int i=0; i<a.size(); ++i)
	{
		result.push_back(a[i] + b[i]);
	}
	return result;
}

std::vector<int32_t> vector_multiply(const std::vector<int32_t>& a, const int32_t b)
{
	std::vector<int32_t> result;
	for(int i=0; i<a.size(); ++i)
	{
		result.push_back(a[i] * b);
	}
	return result;
}

void do_tick(
	registers& real_reg,
	int32_t& ip,
	const std::vector<instruction>& instructions,
	std::map<std::string, operation>& operations,
	const int32_t ip_index,
	int64_t& tick
)
{
	real_reg[ip_index] = ip;
	auto ins = instructions[ip];
	operations[ins.op](ins.a, ins.b, ins.reg_out, real_reg);
	ip = real_reg[ip_index];
	++ip;
	++tick;
}

registers emulate_ticks(
	const registers& reg,
	int64_t tick_limit,
	int32_t ip,
	const int32_t ip_index,
	const std::vector<instruction>& instructions,
	std::map<std::string, operation>& operations
)
{
	auto real_reg = reg;
	int64_t tick=0;
	while(ip >= 0 && ip < instructions.size() && tick < tick_limit)
	{
		do_tick(real_reg, ip, instructions, operations, ip_index, tick);
	}
	return real_reg;
}

struct exit_condition_type
{
	int64_t extra_ticks;
	int out_register;
	registers start_point;
	bool exit_value;
	std::vector<int32_t> diff;
	instruction ins;
	int id;
};

std::vector<exit_condition_type> get_exit_conditions(
	const int32_t loop_length,
	const registers& reg,
	int32_t ip,
	const int32_t ip_index,
	const std::vector<instruction>& instructions,
	std::map<std::string, operation>& operations
)
{
	std::vector<exit_condition_type> result;

	auto test_reg = reg;
	int64_t tick=0;
	int id = 0;
	while(tick < loop_length)
	{
		auto ins = instructions[ip];
		if(
			ins.op == "gtir" ||
			ins.op == "gtri" ||
			ins.op == "gtrr" ||
			ins.op == "eqir" ||
			ins.op == "eqri" ||
			ins.op == "eqrr"
		)
		{
			exit_condition_type condition;
			condition.start_point = test_reg;

			do_tick(test_reg, ip, instructions, operations, ip_index, tick);
			bool out = test_reg[ins.reg_out];
			const auto diff_reg = emulate_ticks(test_reg, loop_length-1, ip, ip_index, instructions, operations);

			condition.ins = ins;
			condition.extra_ticks = tick;
			condition.out_register = ins.reg_out;
			condition.exit_value = !out;
			condition.diff = vector_minus(diff_reg, condition.start_point);
			condition.id = id++;
			result.push_back(std::move(condition));
		}
		else
		{
			do_tick(test_reg, ip, instructions, operations, ip_index, tick);
		}
	}

	return result;
}

int32_t optimise_loops(
	const bool one_by_one,
	const std::vector<state>& states,
	registers& reg,
	const int32_t ip,
	const int32_t ip_index,
	const std::vector<instruction>& instructions,
	std::map<std::string, operation>& operations
)
{
	for(int hop_length = 1; hop_length * 3 + 1 < states.size(); ++hop_length)
	{
		const auto& s_3 = states[states.size() - 1 - hop_length - hop_length - hop_length];
		const auto& s_2 = states[states.size() - 1 - hop_length - hop_length];
		const auto& s_1 = states[states.size() - 1 - hop_length];
		const auto& s_0 = states[states.size() - 1];

		auto loop_length = s_0.tick - s_1.tick;
		if(loop_length > 1000)
			continue;

		auto diff = vector_minus(s_0.reg, s_1.reg);
		if(vector_plus(s_2.reg, diff) != s_1.reg)
			continue;
		if(vector_plus(s_3.reg, diff) != s_2.reg)
			continue;

		std::vector<exit_condition_type> exit_conditions = get_exit_conditions(loop_length, s_2.reg, ip, ip_index, instructions, operations);

		std::cout << (one_by_one ? "One by one " : "Binary search ");
		std::cout << "additive loop detected (" << loop_length << " ticks): ";
		print_registers(s_1.reg);
		std::cout << " => ";
		print_registers(s_0.reg);
		std::cout << std::endl;

		std::cout << exit_conditions.size() << " potential exits" << std::endl;
		int a = 0;
		for(const auto& c : exit_conditions)
		{
			//std::cout << a << ": requires " << c.out_register << " to be " << c.exit_value << " from " << c.start_point[ip_index] << ": ";
			//print_registers(c.diff);
			//print_registers(c.start_point);
			//std::cout << std::endl;
			c.ins.print();
			std::cout << std::endl;
		}

		if(exit_conditions.size())
		{
			int32_t max = 1000000000;
			for(int i=0; i<max; ++i)
			{
				for(auto& c: exit_conditions)
				{
					const auto test_reg = emulate_ticks(c.start_point, 1, c.start_point[ip_index] + 1, ip_index, instructions, operations);
					if(test_reg[c.out_register] == c.exit_value)
					{
						int32_t skipped_ticks = i*loop_length + c.extra_ticks;
						std::cout << "skipped " << i << " loops (" << skipped_ticks << " ticks): ";
						//print_registers(reg);
						std::cout << std::endl;

						reg = test_reg;
						return skipped_ticks;
					}
					c.start_point = vector_plus(c.start_point, c.diff);
				}
			}
		}

		break;
	}

	return 0;
}

int main()
{
	std::map<std::string, operation> operations;
	operations["addr"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] + regs[b];};
	operations["addi"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] + b;};
	operations["mulr"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] * regs[b];};
	operations["muli"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] * b;};
	operations["banr"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] & regs[b];};
	operations["bani"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] & b;};
	operations["borr"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] | regs[b];};
	operations["bori"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] | b;};
	operations["setr"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a];};
	operations["seti"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = a;};
	operations["gtir"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = a > regs[b];};
	operations["gtri"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] > b;};
	operations["gtrr"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] > regs[b];};
	operations["eqir"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = a == regs[b];};
	operations["eqri"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] == b;};
	operations["eqrr"] = [](int a, int b, int reg_out, registers& regs){regs[reg_out] = regs[a] == regs[b];};

	int32_t ip_index;

	std::string line;
	std::getline(std::cin, line);
	std::stringstream line_stream(line);
	std::string declaration;
	line_stream >> declaration >> ip_index;
	//std::cout << "#ip " << ip_index << std::endl;

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

	//std::map<int32_t, std::vector<state>> visits;
	int32_t ip = 0;
	registers real_reg{0, 0, 0, 0, 0, 0};
	int64_t tick=0;
	std::vector<int32_t> values;
	while(ip >= 0 && ip < instructions.size())
	{
		//print_registers(real_reg);
		//visits[ip].emplace_back(tick, real_reg);
		/*
		if(
			instructions[ip].op == "gtir" ||
			instructions[ip].op == "gtri" ||
			instructions[ip].op == "gtrr" ||
			instructions[ip].op == "eqir" ||
			instructions[ip].op == "eqri" ||
			instructions[ip].op == "eqrr"
		)
		{
			auto skipped = optimise_loops(false, visits[ip], real_reg, ip, ip_index, instructions, operations);
			tick += skipped;
			ip = real_reg[ip_index] + 1;
			if(skipped != 0)
				std::cout << "tick: " << tick << std::endl;
		}
		*/
		if(ip == 28)
		{
			int current = real_reg[3];
			if(std::find(values.begin(), values.end(), current) != values.end())
			{
				std::cout << values.back() << std::endl;
				print_registers(real_reg);
				std::cout << std::endl;

				print_registers(real_reg);
				break;
			}
			else
			{
				std::cout << values.size() << std::endl;
				values.push_back(real_reg[3]);
			}
		}
		do_tick(real_reg, ip, instructions, operations, ip_index, tick);
	}

	print_registers(real_reg);
	std::cout << std::endl;

	return 0;
}

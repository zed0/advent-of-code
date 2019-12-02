#include <iostream>
#include <sstream>
#include <map>
#include <deque>
#include <algorithm>

char char_from_pot(bool pot)
{
	return pot ? '#' : '.';
}

bool pot_from_char(char pot)
{
	if(pot == '.')
		return 0;
	if(pot == '#')
		return 1;

	std::stringstream err;
	err << "Unexpected character '" << pot << "'";

	throw std::runtime_error(err.str());
}

void print_state(std::deque<bool> pots)
{
	for(const auto pot : pots)
	{
		std::cout << char_from_pot(pot);
	}
	std::cout << std::endl;
}

std::string pots_to_string(std::deque<bool> pots)
{
	std::string result;
	for(const auto pot : pots)
		result.push_back(char_from_pot(pot));
	return result;
}

int next_state(std::deque<bool>& pots, std::map<std::string, bool>& automata)
{

	std::deque<bool> state = {0, 0, 0, 0, 0};
	int initial_size = pots.size();
	int offset = -2;
	for(int64_t i=-2; i<static_cast<int64_t>(initial_size) + 2; ++i)
	{
		state.pop_front();
		size_t next_char_index = i + 2;
		if(next_char_index >= pots.size())
			state.push_back(0);
		else
			state.push_back(pots[next_char_index]);

		if(i < 0)
		{
			pots.push_front(automata[pots_to_string(state)]);
			i++;
			offset++;
		}
		else if(i < pots.size())
			pots[i] = automata[pots_to_string(state)];
		else
			pots.push_back(automata[pots_to_string(state)]);
	}

	auto is_true = [](const bool a){return a;};
	auto first_false = std::find_if(pots.begin(), pots.end(), is_true);
	offset = std::distance(pots.begin(), first_false) + offset;
	pots.erase(pots.begin(), first_false);
	pots.erase(std::find_if(pots.rbegin(), pots.rend(), is_true).base(), pots.end());

	return offset;
}

int64_t score(const std::deque<bool>& pots, int64_t offset)
{
	int64_t total = 0;
	for(int64_t i=0; i<pots.size(); ++i)
	{
		if(pots[i])
			total += (i + offset);
	}
	return total;
}

int main()
{
	std::string current_line;
	std::getline(std::cin, current_line);
	int64_t iterations = 50000000000;
	//int64_t iterations = 20;

	std::deque<bool> pots;

	{
		std::stringstream line_stream(current_line);
		std::string initial, state;
		line_stream >> initial >> state;
		char pot;
		while(line_stream >> pot)
		{
			pots.push_back(pot_from_char(pot));
		}
	}
	std::getline(std::cin, current_line);

	std::map<std::string, bool> automata;

	while(std::getline(std::cin, current_line))
	{
		std::stringstream line_stream(current_line);
		std::string state;
		std::string arrow;
		char pot;
		line_stream >> state >> arrow >> pot;
		automata[state] = pot_from_char(pot);
	}

	/*
	for(const auto automaton : automata)
	{
		std::cout << automaton.first << ": " << char_from_pot(automaton.second) << std::endl;
	}
	std::cout << std::endl;
	*/

	//print_state(pots);
	int offset = 0;
	int offset_diff = 0;
	uint64_t iteration = 0;

	for(; iteration<=iterations; ++iteration)
	{
		const auto prev_pots = pots;
		const auto prev_offset = offset;
		offset += next_state(pots, automata);
		/*
		std::cout << iteration << ": (" << offset << "): " << score(pots, offset) << "\t";
		for(int j=0; j<offset+10; ++j)
			std::cout << " ";
		print_state(pots);
		*/

		if(prev_pots == pots)
		{
			std::cout << "converged" << std::endl;
			offset_diff = offset - prev_offset;
			break;
		}
	}
	int64_t remaining_iterations = iterations - iteration - 1;
	std::cout << "stopped on iteration " << iteration << " (" << remaining_iterations << " remaining)" << std::endl;
	int64_t final_offset = offset + (remaining_iterations * offset_diff);
	std::cout << "final offset: " << final_offset << " (" << offset << " + " << "(" << remaining_iterations << "*" << offset_diff <<"))" << std::endl;
	std::cout << "final score: " << score(pots, final_offset) << std::endl;;
	return 0;
}

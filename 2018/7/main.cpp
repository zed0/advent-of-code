#include <iostream>
#include <vector>
#include <map>
#include <set>
#include <algorithm>

struct input
{
	char id;
	char requires;

	void print() const
	{
		std::cout << "Step " << requires << " must be finished before step " << id << " can begin." << std::endl;
	}
};

struct job
{
	job() :
		end_time(99999999)
	{
	}

	char id;
	std::set<char> requirements;
	int end_time;
};

int main()
{
	std::string step;
	std::string must;
	std::string be;
	std::string finished;
	std::string before;
	std::string step2;
	std::string can;
	std::string begin;

	std::vector<input> inputs;
	input current_input;
	while(std::cin >> step >> current_input.requires >> must >> be >> finished >> before >> step >> current_input.id >> can >> begin)
	{
		inputs.push_back(current_input);
	}

	std::cout << "Part 1:" << std::endl;

	std::map<char, std::set<char>> requirements;

	for(const auto& in : inputs)
	{
		requirements[in.requires];
		requirements[in.id].insert(in.requires);
	}

	auto requirement_empty = [](const std::pair<char, std::set<char>> requirement){return requirement.second.empty();};
	while(requirements.size())
	{
		/*
		for(const auto& requirement : requirements)
		{
			std::cout << requirement.first << ": ";
			for(const auto& i : requirement.second)
			{
				std::cout << i << ", ";
			}
			std::cout << std::endl;
		}
		*/

		auto next_empty = std::find_if(requirements.begin(), requirements.end(), requirement_empty);

		if(next_empty == requirements.end())
			break;

		char next_key = next_empty->first;
		std::cout << next_key;
		requirements.erase(next_key);

		for(auto& requirement : requirements)
		{
			requirement.second.erase(next_key);
		}
	}
	std::cout << std::endl;


	std::cout << "Part 2:" << std::endl;

	std::map<char, job> jobs;

	for(const auto& in : inputs)
	{
		jobs[in.requires].id = in.requires;
		jobs[in.id].id = in.id;
		jobs[in.id].requirements.insert(in.requires);
	}

	int tick = 0;

	std::map<int, job> in_progress;
	int workers = 5;
	int extra_time = 60;

	auto job_empty = [](const std::pair<char, job>& j){return j.second.requirements.empty();};
	while(jobs.size() || in_progress.size())
	{
		for(int worker=1; worker<=workers; ++worker)
		{
			if(!in_progress.count(worker))
				continue;

			if(in_progress[worker].end_time <= tick)
			{
				for(auto& requirement : jobs)
				{
					requirement.second.requirements.erase(in_progress[worker].id);
				}

				in_progress.erase(worker);
			}
		}

		for(int worker=1; worker<=workers; ++worker)
		{
			if(in_progress.count(worker))
				continue;

			auto next_empty = std::find_if(jobs.begin(), jobs.end(), job_empty);

			if(next_empty == jobs.end())
				break;

			in_progress[worker] = next_empty->second;
			in_progress[worker].end_time = tick + next_empty->second.id - 'A' + 1 + extra_time;

			char next_key = next_empty->second.id;
			jobs.erase(next_key);
		}

		/*
		std::cout << tick << ": ";
		for(int worker=1; worker<=workers; ++worker)
		{
			if(in_progress.count(worker))
			{
				std::cout << worker << ":" << in_progress[worker].id << "(" << in_progress[worker].end_time << ")" << ", ";
			}
		}
		std::cout << std::endl;
		*/

		++tick;
	}

	std::cout << tick - 1 << " ticks" << std::endl;

	return 0;
}

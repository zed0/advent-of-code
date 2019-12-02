#include <iostream>
#include <sstream>
#include <vector>
#include <map>
#include <algorithm>

struct guard
{
	uint64_t id;
	std::vector<int> sleep_map;

	guard()
	{
		id = 0;
		sleep_map.resize(60);
	}

	int max_minute() const
	{
		auto max_minute = std::max_element(sleep_map.begin(), sleep_map.end());
		return *max_minute;
	}

	int minutes_asleep() const
	{
		int sum = 0;
		for(const auto minute : sleep_map)
		{
			sum += minute;
		}
		return sum;
	}

	void print() const
	{
		std::cout << "#" << id << ": " << minutes_asleep() << std::endl;
		std::cout << "\t";
		for(int i=0; i<60; ++i)
		{
			if(sleep_map[i])
				std::cout << sleep_map[i];
			else
				std::cout << ".";
		}
		std::cout << std::endl;
	}
};

static bool guard_compare1(std::pair<uint64_t, guard> a, std::pair<uint64_t, guard> b)
{
	return a.second.minutes_asleep() < b.second.minutes_asleep();
}

static bool guard_compare2(std::pair<uint64_t, guard> a, std::pair<uint64_t, guard> b)
{
	return a.second.max_minute() < b.second.max_minute();
}

int main()
{
	std::vector<std::string> lines;
	std::string current_line;
	while(std::getline(std::cin, current_line))
	{
		lines.push_back(current_line);
	}

	std::sort(lines.begin(), lines.end());

	std::map<uint64_t, guard> guards;
	uint64_t current_id;
	int sleep_start;
	int sleep_end;
	for(const auto& line : lines)
	{
		std::stringstream line_stream(line);
		char bracket;
		char dash;
		char dash2;
		char colon;
		char bracket2;
		int year;
		int month;
		int day;
		int hour;
		int minute;
		line_stream >> bracket >> year >> dash >> month >> dash2 >> day >> hour >> colon >> minute >> bracket2;

		std::string text;
		line_stream >> text;
		if(text == "Guard")
		{
			char hash;
			line_stream >> hash;
			line_stream >> current_id;
			guards[current_id].id = current_id;
		}
		else if(text == "falls")
		{
			sleep_start = minute;
		}
		else if(text == "wakes")
		{
			sleep_end = minute;
			for(int i=sleep_start; i<sleep_end; ++i)
			{
				++guards[current_id].sleep_map[i];
			}
		}
	}

	std::cout << "STRATEGY 1" << std::endl;

	auto max = std::max_element(guards.begin(), guards.end(), guard_compare1);
	auto sleepy = max->second;
	std::cout << "Guard: " << sleepy.id <<"; Minute: " << sleepy.minutes_asleep() << std::endl;
	//sleepy.print();
	auto best_minute = std::max_element(sleepy.sleep_map.begin(), sleepy.sleep_map.end());
	auto minute_number = std::distance(sleepy.sleep_map.begin(), best_minute);
	std::cout << "Most slept minute: " << minute_number << std::endl;
	std::cout << minute_number * sleepy.id << std::endl;

	std::cout << "STRATEGY 2" << std::endl;

	auto max2 = std::max_element(guards.begin(), guards.end(), guard_compare2);
	auto sleepy2 = max2->second;
	std::cout << "Guard: " << sleepy2.id <<"; Minute: " << sleepy2.minutes_asleep() << std::endl;
	auto best_minute2 = std::max_element(sleepy2.sleep_map.begin(), sleepy2.sleep_map.end());
	auto minute_number2 = std::distance(sleepy2.sleep_map.begin(), best_minute2);
	std::cout << "Most slept minute: " << minute_number2 << std::endl;
	std::cout << minute_number2 * sleepy2.id << std::endl;


	return 0;
}

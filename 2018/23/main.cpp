#include <iostream>
#include <vector>
#include <regex>
#include <cmath>
#include <sstream>
#include <algorithm>

struct position
{
	position(
		const int64_t x_,
		const int64_t y_,
		const int64_t z_
	) :
		x(x_),
		y(y_),
		z(z_)
	{
	}

	int64_t x;
	int64_t y;
	int64_t z;
};

std::vector<position> directions()
{
	return {
		{ 0,  0,  0},
		{ 0,  0, -1}, { 0,  0,  1},
		{ 0, -1,  0}, { 0,  1,  0},
		{-1,  0,  0}, { 1,  0,  0}
	};
}

struct nanobot
{
	int64_t x;
	int64_t y;
	int64_t z;
	int64_t r;
	int64_t id;

	std::string to_string() const
	{
		std::stringstream ss;
		ss << x << "," << y << "," << z << ": " << r;
		return ss.str();
	}

	int64_t distance_to(const nanobot& other) const
	{
		return std::abs(x - other.x) + std::abs(y - other.y) + std::abs(z - other.z);
	}
};

struct box
{
	int64_t x;
	int64_t y;
	int64_t z;
	int64_t r;

	std::string to_string() const
	{
		std::stringstream ss;
		ss << x << "," << y << "," << z << ": " << r;
		return ss.str();
	}

	int64_t distance_to(const nanobot& other) const
	{
		return std::abs(x - other.x) + std::abs(y - other.y) + std::abs(z - other.z);
	}

	bool intersects(const nanobot& bot) const
	{
		return distance_to(bot) <= r + bot.r;
	}

	int64_t count_intersects(const std::vector<nanobot>& bots) const
	{
		int64_t total = 0;
		for(const auto& bot : bots)
		{
			total += intersects(bot);
		}
		return total;
	}
};

int main()
{
	std::vector<nanobot> bots;

	std::string line;
	int64_t id = 0;
	while(std::getline(std::cin, line))
	{
		std::smatch m;
		std::regex bot_regex(R"regex(pos=<([\d-]+),([\d-]+),([\d-]+)>, r=([\d-]+))regex");
		std::regex_match(line, m, bot_regex);

		nanobot new_nanobot;
		int64_t x = std::stoi(m[1]);
		int64_t y = std::stoi(m[2]);
		int64_t z = std::stoi(m[3]);
		int64_t r = std::stoi(m[4]);
		new_nanobot.x = x;
		new_nanobot.y = y;
		new_nanobot.z = z;
		new_nanobot.r = r;
		new_nanobot.id = id;
		bots.push_back(new_nanobot);
		id++;
	}

	auto comp_x = [](const nanobot& a, const nanobot& b){return a.x < b.x;};
	auto comp_y = [](const nanobot& a, const nanobot& b){return a.y < b.y;};
	auto comp_z = [](const nanobot& a, const nanobot& b){return a.z < b.z;};

	const auto max_x = std::max_element(bots.begin(), bots.end(), comp_x);
	const auto max_y = std::max_element(bots.begin(), bots.end(), comp_y);
	const auto max_z = std::max_element(bots.begin(), bots.end(), comp_z);
	const auto min_x = std::min_element(bots.begin(), bots.end(), comp_x);
	const auto min_y = std::min_element(bots.begin(), bots.end(), comp_y);
	const auto min_z = std::min_element(bots.begin(), bots.end(), comp_z);

	box bounding;
	bounding.x = (max_x->x + min_x->x)/2;
	bounding.y = (max_y->y + min_y->y)/2;
	bounding.z = (max_z->z + min_z->z)/2;

	std::vector<int64_t> distances;

	auto distance_to = [&bounding](const nanobot& bot){return bounding.distance_to(bot);};
	std::transform(bots.begin(), bots.end(), std::back_inserter(distances), distance_to);
	bounding.r = *std::max_element(distances.begin(), distances.end());

	std::cout << "Bounding box: " << bounding.to_string() << std::endl;

	int64_t tick = 0;
	while(bounding.r >= 1)
	{
		tick++;
		const auto current_count = bounding.count_intersects(bots);
		if(tick % 100 == 0)
		{
			std::cout << tick << ": " << bounding.to_string() << ": " << current_count << std::endl;
		}

		int64_t lower_bound_r = 0;
		int64_t upper_bound_r = bounding.r;
		while(upper_bound_r > lower_bound_r + 1)
		{
			bounding.r = (upper_bound_r + lower_bound_r)/2;
			if(bounding.count_intersects(bots) == current_count)
			{
				upper_bound_r = bounding.r;
			}
			else
			{
				lower_bound_r = bounding.r;
			}
		}
		bounding.r = upper_bound_r;

		box best_next;
		int64_t best_num = 0;

		std::vector<box> nexts;
		for(const auto direction : directions())
		{
			auto next_bounding = bounding;
			next_bounding.x += direction.x;
			next_bounding.y += direction.y;
			next_bounding.z += direction.z;
			next_bounding.r -= 1;
			nexts.push_back(next_bounding);
		}
		for(const auto direction : directions())
		{
			auto next_bounding = bounding;
			next_bounding.x += (direction.x * 100);
			next_bounding.y += (direction.y * 100);
			next_bounding.z += (direction.z * 100);
			next_bounding.r -= 100;
			nexts.push_back(next_bounding);
		}
		for(const auto direction : directions())
		{
			auto next_bounding = bounding;
			next_bounding.x += (direction.x * 1000);
			next_bounding.y += (direction.y * 1000);
			next_bounding.z += (direction.z * 1000);
			next_bounding.r -= 1000;
			nexts.push_back(next_bounding);
		}
		for(const auto direction : directions())
		{
			auto next_bounding = bounding;
			next_bounding.x += (direction.x * 1000000);
			next_bounding.y += (direction.y * 1000000);
			next_bounding.z += (direction.z * 1000000);
			next_bounding.r -= 1000000;
			nexts.push_back(next_bounding);
		}

		auto best_comp = [&bots](const box& a, const box& b){
			const auto count_a = a.count_intersects(bots);
			const auto count_b = b.count_intersects(bots);
			if(count_a == count_b)
			{
				if(a.r == b.r)
					return std::abs(a.x) + std::abs(a.y) + std::abs(a.z) > std::abs(b.x) + std::abs(b.y) + std::abs(b.z);
				else
					return a.r > b.r;
			}
			else
			{
				return count_a < count_b;
			}
		};

		auto next = std::max_element(nexts.begin(), nexts.end(), best_comp);

		bounding = *next;

		auto is_outside = [&bounding](const nanobot& bot){return !bounding.intersects(bot);};
		bots.erase(std::remove_if(bots.begin(), bots.end(), is_outside), bots.end());
	}
	std::cout << tick << ": " << bounding.to_string() << std::endl;

	return 0;
}

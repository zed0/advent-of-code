#include <iostream>
#include <regex>
#include <vector>
#include <string>
#include <limits>

typedef std::vector<std::vector<int>> canvas_type;
struct star_type
{
	int64_t x;
	int64_t y;
	int64_t dx;
	int64_t dy;

	void tick()
	{
		x += dx;
		y += dy;
	}

	void backtick()
	{
		x -= dx;
		y -= dy;
	}
};

bool comp_x(const star_type& a, const star_type& b){return a.x < b.x;};
bool comp_y(const star_type& a, const star_type& b){return a.y < b.y;};

int64_t size_x(const std::vector<star_type>& stars)
{
	auto min_x = std::min_element(stars.begin(), stars.end(), &comp_x);
	auto max_x = std::max_element(stars.begin(), stars.end(), &comp_x);

	auto size_x = max_x->x - min_x->x;

	return size_x;
}


int64_t size_y(const std::vector<star_type>& stars)
{
	auto min_y = std::min_element(stars.begin(), stars.end(), &comp_y);
	auto max_y = std::max_element(stars.begin(), stars.end(), &comp_y);
	auto size_y = max_y->y - min_y->y;

	return size_y;
}

void print(const std::vector<star_type>& stars)
{
	canvas_type canvas;
	canvas.resize(size_y(stars) + 1, std::vector<int>(size_x(stars) + 1, 0));

	auto min_y = std::min_element(stars.begin(), stars.end(), &comp_y);
	auto min_x = std::min_element(stars.begin(), stars.end(), &comp_x);
	for(const auto& star : stars)
	{
		const auto y = star.y - min_y->y;
		const auto x = star.x - min_x->x;
		canvas[y][x] = 1;
	}

	for(const auto& row : canvas)
	{
		for(const auto& cell : row)
		{
			if(cell)
				std::cout << "#";
			else
				std::cout << ".";
		}
		std::cout << std::endl;
	}
}

int main()
{
	std::vector<star_type> stars;

	std::string input;
	while(std::getline(std::cin, input))
	{
		star_type star_in;

		std::smatch m;
		std::regex input_regex(R"regex(position=<\s*([0-9\-]+),\s*([0-9\-]+)> velocity=<\s*([0-9\-]+),\s*([0-9\-]+)>)regex");
		std::regex_match(input, m, input_regex);
		star_in.x  = std::stoi(m[1]);
		star_in.y  = std::stoi(m[2]);
		star_in.dx = std::stoi(m[3]);
		star_in.dy = std::stoi(m[4]);

		stars.push_back(star_in);
	}

	uint64_t current_area = std::numeric_limits<uint64_t>::max();
	int i=0;
	while(true)
	{
		++i;
		for(auto& star : stars)
			star.tick();

		auto area = size_x(stars) * size_y(stars);
		if(area > current_area)
		{
			--i;
			for(auto& star : stars)
				star.backtick();
			break;
		}

		current_area = area;
	}

	std::cout << i << " seconds:" << std::endl;
	print(stars);

	return 0;
}

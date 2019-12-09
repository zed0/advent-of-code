#include <iostream>
#include <string>
#include <sstream>
#include <vector>

struct triangle_t
{
	int a;
	int b;
	int c;
	triangle_t(int a_, int b_, int c_) :
		a(a_),
		b(b_),
		c(c_)
	{
	}
};

int main()
{
	int count = 0;

	std::string in;
	std::vector<triangle_t> triangles;

	while(std::getline(std::cin, in))
	{
		std::istringstream iss(in);
		int a_1,a_2,a_3;
		int b_1,b_2,b_3;
		int c_1,c_2,c_3;
		iss >> a_1 >> a_2 >> a_3;
		std::getline(std::cin, in);
		iss = std::istringstream(in);
		iss >> b_1 >> b_2 >> b_3;
		std::getline(std::cin, in);
		iss = std::istringstream(in);
		iss >> c_1 >> c_2 >> c_3;

		triangles.emplace_back(a_1, b_1, c_1);
		triangles.emplace_back(a_2, b_2, c_2);
		triangles.emplace_back(a_3, b_3, c_3);
	}

	for(const auto tri : triangles)
	{
		if((tri.a + tri.b > tri.c)
			&& (tri.a + tri.c > tri.b)
			&& (tri.b + tri.c > tri.a)
		)
		{
			++count;
		}
	}

	std::cout << count << std::endl;
}

#include <string>
#include <iostream>
#include <complex>
#include <deque>

using namespace std;
using namespace complex_literals;
int z()
{
	complex<int> d{0,1};
	deque<complex<int>> p{{0,0}};
	string i;
	while(cin>>i)
	{
		istringstream c(i);
		char a;
		int b;
		c>>a>>b;
		d*=a==82?-1i:1i;
		for(;b--;)
		{
			auto n=p[0]+d;
			if(count(begin(p),end(p),n))
				return abs(n.real())+abs(n.imag());
			p.push_front(n);
		}
	}
}

int main()
{
	cout<<z()<<endl;
}

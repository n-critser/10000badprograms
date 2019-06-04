#include<iostream>

int gcd(int a, int b);

int main(){
  int a,b,c;
  std::cout << "enter integers a,b,c hitting enter each time"<<std::endl;
  std::cin >>a >>b >>c;
  
  std::cout<<"a:"<< a << ","<< "b:"<<b <<","<< "c:"<<c << std::endl;
  int gcdd = gcd(a,b);
  std::cout << "gcdd(a,b):"<< gcdd << std::endl;

  bool divisible = (c % gcdd) == 0 ;

  std::cout << "divisible:"<< divisible << std::endl;
  return 0;
}


int gcd(int a, int b){

  int r=0;

  int max = a > b ? a : b;
  int min = a <= b? a : b;
  int res = -1;
  int minus = -1;
  do {
    r = max % min;
    if (r == 0){
      res = min;
    } else {
      minus = max - min;
      max = min > minus? min : minus;
      min = min <= minus? min : minus;
    }
  }while (r > 0);
  return res;
}

#include <iostream>
#include <lib.hpp>

bool is_congruent(int left , int right, int mod);

int main()
{
  int congruent = is_congruent(2,3,5);
    dummy();
    std::cout << "congruent:" << congruent << std::endl;
    return 0;
}

bool is_congruent(int left , int right, int mod){

  int cong = (left - right) % mod ;
  std::cout << "cong:" << cong << std::endl;
  bool res=false;
  if ( cong == 0){
    res = true;
  }
  
  return res;
}

//#include<stdio.h>
#include<iostream>


unsigned gcd(int left, int right);

int main(int arc,char**argv){

  std::cout << "hello world" << '\n';

  for (int i = 6; i> 0; i--){
    std::cout << "gcd :(32 ,"<<  i << ") ="<< gcd(32,i) << '\n';
  }
  return 0;
}

unsigned gcd(int left, int right){

  int r=-1;
  int max = left > right ? left: right;
  int min = left <= right? left: right;
  int gcd = 0;
  do {
    // std::cout<< "max:"<<max <<","<<"min:"<<min<< '\n';
    if ( (r = (max % min)) == 0){

      gcd = min;
    } else {
      int minus= max - min;
      max = minus > min ? minus: min;
      min = minus <= min? minus: min;

    }
    
  } while (r !=0);
  
  return gcd;
}

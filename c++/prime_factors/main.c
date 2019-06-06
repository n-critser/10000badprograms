#include<stdio.h>
#include<string.h>
#include<stdlib.h>
unsigned gcd(int left, int right);
int is_factor(unsigned num , unsigned factor);
const char* get_factorization(unsigned num,char* buffer, unsigned len );
int main(int arc,char**argv){

  printf("hello world");

  char buf[256];
  memset(buf,0,256);
  get_factorization(67108865,buf,255);
  printf(buf);
  printf("\n");

  return 0;
  
}


unsigned gcd(int left, int right){

  int r=-1;
  int max = left > right ? left: right;
  int min = left <= right? left: right;
  int gcd = 0;
  do {
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

const char* get_factorization(unsigned num,char* buffer, unsigned len ){
  int i = 2;
  int left=0;
  int quotient=num;
  for (;i <= num;i++){
    if (is_factor(quotient,i)){
      // count the exponent of factor
      unsigned exp=0;

      do{
        quotient= quotient / i;
        exp++;

      }while(is_factor(quotient,i));
        
      printf ("%d is a factor\n",i);
      int i_len = snprintf( NULL, 0, "%d", i );
      char* i_str = malloc( i_len + 1 );
      int exp_len = snprintf(NULL, 0 , "^%d-",exp);
      char* exp_str = malloc(exp_len +1);
      snprintf( i_str, i_len + 1, "%d", i );
      snprintf( exp_str, exp_len + 1, "^%d-", exp );
      strncpy(buffer+left,i_str,i_len);      
      left+=i_len;
      strncpy(buffer+left,exp_str,exp_len);
      left+=exp_len;
      free(i_str);
      free(exp_str);

    }
  }

  return buffer;
  
}


int is_factor(unsigned num , unsigned factor){
  if (!(num > 0 && factor > 0)){
    return 0;
  } else{
    return (num % factor ) == 0;
  }
}

#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include "../cardano.h"

int8_t bytes_to_hex( uint8_t*, unsigned int, char *);

int main() {

  char *plain1 = "Hello World!";
  char *ebuffer = (char *)malloc(1000);
  memset(ebuffer,'\0',1000);
 
  printf("\nData to encode: %s \n",plain1);
  my_b58_encode((uint8_t *)plain1,strlen(plain1),ebuffer);

  printf("Encoded: %s \n",ebuffer);

  uint8_t *dbuffer = (uint8_t *)malloc(1000);
  memset(dbuffer,'\0',1000);

  my_b58_decode(ebuffer,(uint8_t *)dbuffer);
   
  printf("Decoded again: %s \n\n",dbuffer);

  free(ebuffer);
  free(dbuffer);
}

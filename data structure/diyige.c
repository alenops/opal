#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#define maxsize 100
using namespace std;

typedef struct{
  int data[maxsize];
  int length;
}Sqlist;

void reverse(Sqlist &A)
{
 int i=0,j=A.length-1,temp=0;
 while(i<j)
  {
      temp=A.data[i];
      A.data[i]=A.data[j];
      A.data[j]=temp;
      i++;
      j--;
  }
}

int main()
{
    Sqlist A;
    int i;
    scanf("%d",&A.length);
    for(i=0;i<A.length;i++) scanf("%d",&A.data[i]);
    reverse(A);
    for(i=0;i<A.length;i++) printf("%d ",A.data[i]);
    return 0;
}

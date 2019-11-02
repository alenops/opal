#include <stdio.h>
#include <stdlib.h>
#include <time.h>
int main()
{

    int rands,c=0,c1=0;
    srand((signed)time(NULL));
    for(int i=0;i<15000;i++){
    rands = rand();
    if(rands%2==0){c++;}
    else c1++;
    printf("%d\n", rands);
    }
    printf("\n偶数为：%d\n奇数为：%d\n",c,c1);
    return 0;

}


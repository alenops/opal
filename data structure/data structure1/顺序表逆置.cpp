//顺序表逆置的程序代码
#include<stdio.h>
#include<malloc.h>
//顺序表结构类型定义
typedef char datatype;
const int maxsize=1024;
typedef struct
{ datatype data[maxsize];
  int last;
}sequenlist;
void create(sequenlist*&);
void print(sequenlist*);
void invert(sequenlist*);

int  main()
{
	sequenlist*L;
	create(L);//建立顺序表
	print(L);//输出顺序表
	invert(L);//调用顺序表逆值的函数
	print(L);//输出顺序表
}

//建立顺序表
void create(sequenlist*&L)
{
	L=(sequenlist*)malloc(sizeof(sequenlist));
	L->last=0;
	char ch;
	while((ch=getchar())!='*')
	{   
		L->last++;
		L->data[L->last]=ch;
	}
}

//输出顺序表
void print(sequenlist*L)
{
	for(int i=1;i<=L->last;i++)
		printf("%2c",L->data[i]);
	printf("\n");
}

//顺序表逆置
void  invert(sequenlist* L){
    int  temp;  //辅助变量
    for(int i=0;i<L->last/2;i++){
        temp=L->data[i];  //交换 L.data[i]与 L.data[L.length-i-1]
        L->data[i]=L->data[L->last-i-1];
        L->data[L->last-i-1]=temp;
    }
}
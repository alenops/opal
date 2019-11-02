//希尔排序的程序代码
#include<stdio.h>
//顺序表结构类型定义
typedef int datatype;
typedef struct{
	int key;
	datatype data;
}rectype;
const int N=10;
const int D1=5;

void create(rectype[],int);
void print(rectype[],int);
void shellsort(rectype[],int[]);

void main()
{
    rectype r[N+D1];//D1个元素存放监视哨，N个元素存放记录
    int d[3]={5,3,1};//设置3趟的增量	
	create(r,N);//建立存放记录的顺序表
	printf("排序前的数据：");
	print(r,N);//输出排序前的记录表
	shellsort(r,d);//希尔排序
	printf("排序后的数据：");
	print(r,N);//输出排序后的记录表
}

//建立顺序表
void create(rectype r[],int n)
{
	printf("输入10个整型数：");
	for(int i=0;i<n;i++)
		scanf("%d",&r[D1+i].key);
}

//输出顺序表
void print(rectype r[],int n)
{
	for(int i=0;i<n;i++)
		printf("%5d",r[D1+i].key);
	printf("\n");
}

//添加希尔排序算法


#include<stdio.h>
#include<cs50.h>
#include<math.h>

int main(void) {
    double a;
    while(true)
    {
        scanf("%lf",&a);
        if(a<0)
        {
            printf("Please enter the input again\n");
        }
        else
        break;
    }
    int c = a*100;
    int q = c/25;
    c=c%25;
    int d = c/10;
    c=c%10;
    int n = c/5;
    c=c%5;
    int p = c%5;
    printf("%d\n",q+d+n+p);

}
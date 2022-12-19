#include <stdio.h>
#include <cs50.h>

int main()
{
    int n;
    while(true)
    {
        printf("Height : ");
        scanf("%d",&n);
        if(n<=0 || n>8)
        {
            printf("Height : %d\n",n);
        }
        else
        break;
    }
    for(int i=0;i<n;i++)
    {
        for(int j=0;j<=i;j++)
        {
            printf("#");
        }
        printf("\n");
    }
    printf("\n");
    printf("Now right handed pyramid\n");
    for(int i=0;i<n;i++)
    {
        for(int k=n;k>=i;k--)
        {
            printf(" ");
        }
        for(int j=0;j<=i;j++)
        {
            printf("#");
        }
        printf("\n");
    }
}
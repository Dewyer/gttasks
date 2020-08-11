function s(x,n,q)
{
    let m = 10e6
    let p=i=0
    for (;i<m;i++)
    {
        let c=k=0
        for (; k < n; k++)
        {
            if (Math.random() < x)
            {
                c++;
            }
        }
        p+=c<=2
    }
    console.log(`${q}:${p}/${m}=>${Math.round(p/m*100)}%`);
}

s(0.32,6,"Hos");
s(0.40,5,"Ben");
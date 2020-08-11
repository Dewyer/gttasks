
function doesLive(xx,nn)
{
    let cc = 0
    for (let kk = 0; kk <nn ;kk++)
    {
        if (Math.random() <= xx)
        {
            cc ++;
        }
    }
    return cc <= 2;
}

function simful(x,n,nev)
{
    let max = 10e6;
    let pp = 0;
    for (let ii = 0; ii < max; ii++)
    {
        pp += doesLive(x, n) ? 1 : 0;
    }
    console.log(`${nev}: ${pp}/${max} = ${pp / max}`);
}
simful(0.32,6,"Hos");
simful(0.40, 5, "Ben");
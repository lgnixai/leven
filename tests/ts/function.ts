const a=[20,100,4];
function Sum(x: number, y: number) : number {
    return x + y+a[0];
}
function disyplay():number{
    return Sum(2,a[1]);
}
//Sum(2,3);
let c=disyplay();
print(c);
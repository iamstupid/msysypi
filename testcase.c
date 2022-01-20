int a(int i,int j){
    if(i>=1) return a(i-1,a(i-1,j+1));
    return j+1;
}
int main(){
    putint(5,0);
}
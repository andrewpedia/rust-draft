package main
type Something struct{
	aa int;
}

func do() *Something{
	var a = Something{
		aa:1,}
		;
	return &a;
}
func main(){
	var some = do();

		some.aa=100;
	println("some.a:{}",some.aa);
}

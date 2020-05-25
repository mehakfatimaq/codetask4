fn main() {
   for i in (2..100).rev() {
      println!("{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n", i, i, i-1);
} 
println!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n
No more bottles of beer on the wall, no more bottles of beer. \nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}

//this is a recursive function which takes four parameters i.e
//1.) n = no of disks.
//2.) from= specifies the initial position of the disks.
//3.) to= specifies the final position of the disks.
//4.) via= a helper variable (third pole)

fn towerofhanoi(n: i32, from: i32, to: i32, via: i32) {
    if n > 0 {
  //shifting disks from pole 1 to 3.
        towerofhanoi(n - 1, from, via, to);
        println!("Move disk from pole {} to pole {}", from, to);
  //shifting disks from pole 3 to 2.
        towerofhanoi(n - 1, via, to, from);
    }
  }
  
  #[cfg(test)]
  mod test {
      use super::*;
    #[test]
    fn towerofhanoi(4,1,2,3) 
        
  }

  
  //Time complexity of the above program is approximately (2^n).
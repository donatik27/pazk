// placeholder type aliases
type Bit= u8;
type Bitvec = Vec<Bit>;
type Field = usize;
type Rvec = Vec<usize>;

fn compute_ml_ext<F>(v: Bitvec) -> F
where
    F: Fn(Bitvec) -> usize,
{
    todo!();
}

fn lagrange(w: Bitvec, r: Rvec){
    todo!();
}


/// Compute f^~(r) in O(n log n) time and O(log n) space with a streaming pass
fn lemma_37<F>(f: F, r: Rvec)
where
    F: Fn(Bitvec) -> Bit,
{
    let mut f_out = |x:Bitvec| 0;
    for bit in r{
        f_out = f_out(r)+ f(bit)*lagrange(w,r)

    }
    todo!();
}

fn lemma_38<F>(f: F, r: Bitvec)
where
    F: Fn(Bitvec) -> Bit,
{
    todo!();
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_ml_ext() {
        todo!()
    }

    fn test_exercise_34(){
        todo!()
    }
}
use crate::ColorRGB;

impl<'a, T> super::Blur for T
where
    T: IntoIterator<Item = &'a mut ColorRGB>,
{
    fn blur(self, blur_amount: u8) {
        let keep: u8 = 255 - blur_amount;
        let seep: u8 = blur_amount >> 1;
        let mut carry: ColorRGB = ColorRGB::Black;
        let mut iter = self.into_iter().peekable();
        loop {
            let cur = iter.next();
            let nxt = iter.peek();
            if let Some(i) = cur {
                let mut cur: ColorRGB = *i;
                cur.scale(keep);
                cur += carry;
                if let Some(nxt) = nxt {
                    let mut part: ColorRGB = **nxt;
                    part.scale(seep);
                    cur += part;
                    carry = part;
                }
                *i = cur;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Blur;
    use super::*;
    use crate::HSV;

    #[test]
    fn blur_test() {
        let mut arr = [
            ColorRGB::Black,
            ColorRGB::Red,
            ColorRGB::BlueViolet,
            ColorRGB::Yellow,
        ];

        println!("{:?}", arr);
        for _ in 0..4 {
            arr.blur(64);
            println!("{:?}", arr);
        }
    }
}

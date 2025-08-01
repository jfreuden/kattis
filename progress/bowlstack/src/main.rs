/// Bowlstack is a "Hard" (6.1 point) problem on Kattis. https://open.kattis.com/problems/bowlstack

fn read_str() -> String {
    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");
    response.trim_end().to_string()
}

fn read_one<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|tok| tok.parse::<T>().expect("Failed to parse input"))
        .collect()
}

fn try_read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> Result<[T; K], E>
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    read_vec::<T>().try_into()
}

fn read_array<T: std::str::FromStr, const K: usize, E: std::fmt::Debug>() -> [T; K]
where
    T::Err: std::fmt::Debug,
    [T; K]: TryFrom<Vec<T>, Error = E>,
{
    try_read_array().unwrap()
}

type BowlInt = u32;
type BowlFloat = f32;

#[derive(Debug, Copy, Clone)]
struct Bowl {
    height: BowlInt,
    bottom_radius: BowlInt,
    top_radius: BowlInt,
}

impl From<[BowlInt; 3]> for Bowl {
    fn from([height, bottom_radius, top_radius]: [BowlInt; 3]) -> Self {
        Bowl {
            height,
            bottom_radius,
            top_radius,
        }
    }
}

impl Bowl {
    fn slope(&self) -> BowlFloat {
        self.height as BowlFloat / (self.top_radius - self.bottom_radius) as BowlFloat
    }
}

fn main() {
    let test_cases: u32 = read_one();
    let mut case_bowls: Vec<Vec<Bowl>> = Vec::with_capacity(test_cases as usize);
    for _ in 0..test_cases {
        let num_bowls: u32 = read_one();
        let mut bowls: Vec<Bowl> = Vec::with_capacity(num_bowls as usize);

        for _ in 0..num_bowls {
            bowls.push(Bowl::from(read_array()))
        }
        case_bowls.push(bowls);
    }

    for mut bowls in case_bowls {
        println!("{:?}", bowls);
        println!("{:?}", bowls.iter().map(|x|x.bottom_radius as BowlFloat).collect::<Vec<BowlFloat>>());
        println!("{:?}", bowls.iter().map(Bowl::slope).collect::<Vec<BowlFloat>>());


        bowls.sort_by(|a, b| { BowlFloat::total_cmp(&a.slope(), &b.slope()) });
        let mut stackheight = solve(&mut bowls);
        println!("{:?}", stackheight);

        bowls.reverse();
        stackheight = solve(&mut bowls);
        println!("{:?}", stackheight);
    }




        /*
        y1 = slope1 * x1 + b1
        y2 = slope2 * x2 + b2
        x1 = x2 = xc
            where
            r1 <= x1 <= R1 (or maybe unbounded, for when the bowl just sits there)
            r2 <= x2 <= R2 (same caveat)
        so
        y1 = slope1 * rc + b1
        y2 = slope2 * rc + b2
        and since
        y1 = y2 = yc
        then
        slope1 * rc + b1 = slope2 * rc + b2
        (slope1 - slope2) * rc = b2 - b1
        we can say that (b2 - b1) = gap    NOOOO. b2 and b1 are the intercepts, so they are wider than the real gap. see below
        (slope1 - slope2) * rc = gap       NOOOO.

        (note: it might be b1 - b2 or something. Don't have paper and don't wanna mess with
        re-deriving pt-slp form lol.

        okay, we know that we could say b1 = 0. alternatively we could say that b1 is whatever makes
        x = r1 at y = 0. ( i like this better but it's slightly more math )

        the trick here is to then figure out the radius of contact with only the information,
        and without needing the gap itself

        so then the stack height is the sum of all the gaps plus the height of the last bowl


        alrighty sooooo

        y1(r1) = 0 = slope1 * r1 + b1
        b1 = -slope1 * r1

        y2(r2) = gap = slope2 * r2 + b2          THIS WAS WRONG. y2(r2) isn't gap. gap = y2(rc)
        substitute gap
        gap = (slope1 - slope2) * rc = slope2 * r2 + b2
        (slope1 - slope2) * rc = slope2 * r2 + b2

        (slope1 - slope2) * rc - slope2 * r2 = b2
        and
        (slope1 - slope2) * rc = b2 - b1


        slope1 * r1 - slope2 * r2 = 0
        slope1 * r1 = slope2 * r2        This feels wrong



        okay let's start back at here:
        slope1 * rc + b1 = slope2 * rc + b2
        with
        b1 = -slope1 * r1


        slope1 * rc + b1 = slope2 * rc + b2
        slope1 * rc + (-slope1 * r1) = slope2 * rc + b2
        slope1 * rc - slope1 * r1 = slope2 * rc + b2
        (slope1 - slope2) * rc = slope1 * r1 + b2

        In theory, with the contact point and slopes I should be able to find the gap, I suppose.

        y2(rc) = gap = slope2 * rc + b2

        which means that
        slope1 * rc + b1 = slope2 * rc + b2
        becomes
        slope1 * rc + b1 = (gap)


        y1(rc) is also = gap = sl.... wow gap = gap. wow.

        y2(r1)
        again i've fucked up. I've been mixing yc and gap. I need to re-derive and clarify


         ----------      y2(R2)
        _          _     y1(R1)
          X      X       y1(rc) = y2(rc) = yc

            ----         y2(r2) = gap                   y2(r2) = gap because y1(r1) = 0

            ____         y1(r1) = 0

        switching to slopeX = mX, so slope1 = m1 and slope2 = m2 (y = mx + b) style
        y1(r1) = 0   = m1 * r1 + b1
        y2(r2) = gap = m2 * r2 + b2
        y1(rc) = m1 * rc + b1
        y2(rc) = m2 * rc + b2
        y1(rc) = y2(rc)
        --  not sure I need the below --
        y1(R1) = m1 * R1 + b1
        y1(R1) = h1
        R1 from slope
        y2(R2) = m2 * R2 + b2
        y2(R2) = h2
        R2 from slope


        using the top relations only:
        y1(r1) = 0   = m1 * r1 + b1
        y1(rc) = m1 * rc + b1
        y1(rc) = y2(rc) = yc
        y2(rc) = m2 * rc + b2
        y2(r2) = gap = m2 * r2 + b2

        b1 = -1 * m1 * r1
        so
        y1(rc) = m1 * rc - m1 * r1
        y1(rc) = yc = y2(rc)
        so
        m1 * rc - m1 * r1 = m2 * rc + b2


        not sure how to untangle from here.
        maybe find yc's r (rc) in terms of vertical distance from r2?
        ( y2(rc) - y2(r2) ) / ( rc - r2 ) = m2
        okay I think that's it

        ( yc - gap ) / ( rc - r2 ) = m2
        since yc = m1 * rc + b1 then
        ( m1 * rc + b1 ) = m2 * ( rc - r2 )
        since b1 = -1 * m1 * r1 then
        m1 * rc - m1 * r1 = m2 * ( rc - r2 )
        m1 * rc - m1 * r1 = m2 * rc - m2 * r2
        m1 * rc - m2 * rc = m1 * r1 - m2 * r2
        (m1 - m2) * rc = (m1 * r1 - m2 * r2)
        rc = (m1 * r1 - m2 * r2) / (m1 - m2)


        returning to
        m1 * rc - m1 * r1 = m2 * rc + b2
        (m1 - m2) * rc = m2 * rc + m1 * r1 + b2 -> note: we are aiming to find b2 to plug in for gap
        plugging in rc, the first (m1 - m2) cancels
        (m1 * r1 - m2 * r2) = m2 * ((m1 * r1 - m2 * r2) / (m1 - m2)) + m1 * r1 + b2
        (m1 * r1 - m2 * r2) = (m2 / (m1 - m2)) * (m1 * r1 - m2 * r2) + m1 * r1 + b2
        (1 - (m2 / (m1 - m2))) * (m1 * r1 - m2 * r2) = m1 * r1 + b2
        so
        b2 = (1 - (m2 / (m1 - m2))) * (m1 * r1 - m2 * r2) - m1 * r1
        b2 = ((m1 - m2)/(m1 - m2) - m2 / (m1 - m2)) * (m1 * r1 - m2 * r2) - m1 * r1
        b2 = ((m1 - 2 * m2) / (m1 - m2)) * (m1 * r1 - m2 * r2) - m1 * r1
        that was easy


        now:
        gap = m2 * r2 + b2
        so

        gap = [m2 / (m1 - m2)] * (m2 * r2 - m1 * r1)





         */


        /*
        y1(r1) = 0   = m1 * r1 + b1
        y1(rc) = m1 * rc + b1
        y1(rc) = y2(rc) = yc
        y2(rc) = m2 * rc + b2
        y2(r2) = gap = m2 * r2 + b2
        1. ( yc - gap ) / ( rc - r2 ) = m2
        2. ( yc - 0 ) / ( rc - r1 ) = m1

        continuing with (2)
        yc = m1 * (rc - r1)
        now work on (1)

        ( yc - gap ) = m2 * ( rc - r2 )
        gap = yc - m2 * (rc - r2)
        3. gap = m1 (rc - r1) - m2 * (rc - r2)

        still has rc. Low chance I did it right before, but let's try it:
        rc = (m1 * r1 - m2 * r2) / (m1 - m2)
        makes
        gap = m1 (rc - r1) - m2 * (rc - r2)
        gap = m1 * rc - m1 * r1 - m2 * rc - m2 * r2
        gap = m1 * rc - m2 * rc - m1 * r1 - m2 * r2
        4. gap = (rc) * (m1 - m2) - m1 * r1 - m2 * r2
        gap = (m1 * r1 - m2 * r2) - m1 * r1 - m2 * r2
        gap = - m2 * r2


        yeah fuck that



        y1(r1) = 0   = m1 * r1 + b1
        y1(rc) = m1 * rc + b1
        y1(rc) = y2(rc) = yc
        y2(rc) = m2 * rc + b2
        y2(r2) = gap = m2 * r2 + b2

        plugging this into the CAS (yeah, yeah, i know) yields
        rc = (b2 + m1 * r1) / (m1 - m2)
        b1 = -m1 * r1

        gap = (rc) * (m1 - m2) - m1 * r1 - m2 * r2
        gap = (b2 + m1 * r1) - m1 * r1 - m2 * r2
        gap = b2 - m2 * r2

        b2 = gap + m2 * r2

         */

        /*
        IMPORTANT! Can't just sum the gaps, then add the last height.
        Someone might have a pot that they put a bunch of short bowls into.
        as such, yes keep summing the gaps (because it's the new floor for each one).
        but at each stage, look at the top height of the item, and save it as a new max if it's taller
         */







        // TODO: solve the basic trig to determine the stacking height
        // sort by slope, then check whether increasing or decreasing slope is the answer
        // sorting by slope is easy
        // getting the final heights will require contact math
        // but it's "simply" find two circles that are congruent (or such that the smaller bowl sits inside)
        //
        // question:
        // does increasing slope always map to increasing radii and vice versa or no?
        // I would assume not.
        //
        // the bottom is always smaller than the top, at least.

}

fn solve(bowls: &mut Vec<Bowl>) -> f32 {
    let mut stackheight = BowlFloat::default();
    let mut highestbottom = BowlFloat::default();
    for window in bowls.windows(2) {
        let [bottom, top]: [Bowl; 2] = window.try_into().unwrap();
        let m1 = bottom.slope();
        let m2 = top.slope();
        let r1 = bottom.bottom_radius.clone() as BowlFloat;
        let r2 = top.bottom_radius.clone() as BowlFloat;

        let gap = m2 * r2 - m1 * r1;
        highestbottom += gap;
        stackheight = BowlFloat::max(highestbottom + top.height as f32, stackheight);
    }
    stackheight
}

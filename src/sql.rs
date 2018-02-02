
static Q0_SQL :&'static str = "
select
 age, sum(weight) as total_weight
from cats group by age having sum(weight) > 12;";

static Q1_SQL :&'static str = "select name, sum(weight) 
over (order by name) as running_total_weight 
from cats order by name";

static Q2_SQL :&'static str = "
select name, breed, 
sum(weight) over (partition by breed order by name) as running_total_weight
from cats ";

static Q3_SQL :&'static str = "
select row_number() 
over (partition by breed order by name) as num_cats_of_breed, 
name ,breed from cats order by name";

static Q4_SQL :&'static str = "
select 
rank() over (partition by breed order by weight DESC) as ranking,
name, breed, weight
from cats order by ranking, weight DESC";

static Q5_SQL :&'static str = "
select
 name, weight, ntile(4) over ( order by weight) as weight_quartile
       from  cats 
       ";

static Q6_SQL :&'static str = "
select 
dense_rank() over (order by age DESC) as r, name,age
 from cats order by r";

static Q7_SQL :&'static str = "
select name, weight, 
      lag(weight, 1) over (order by weight) - weight as target_weight
      from cats order by weight";

static Q8_SQL :&'static str = "
    select name, breed, weight,
lag(weight, 1) over (partition by breed order by weight) - weight as target_weight
from cats order by weight ";

static Q9_SQL :&'static str = "
select name, color,
first_value(weight) over (partition by color order by weight) as lowest_weight_by_color
from cats ";

static Q10_SQL :&'static str = "
select name, weight, 
       ntile(2) over ntile_window as by_half,
       ntile(3) over ntile_window as thirds,
       ntile(4) over ntile_window as quart
              from cats
              window ntile_window AS
                       ( ORDER BY weight)
     order by weight";


pub fn get_sql_for_q(s: &String) -> (&str, &str) {
    match s.as_ref() {
        "q0" => (Q0_SQL, "group by"),
        "q1" => (Q1_SQL, "over"),
        "q2" => (Q2_SQL, "partition by"),
        "q3" => (Q3_SQL, "row_number"),
        "q4" => (Q4_SQL, "rank"),
        "q5" => (Q5_SQL, "ntile"),
        "q6" => (Q6_SQL, "dense_rank"),
        "q7" => (Q7_SQL, "lag"),
        "q8" => (Q8_SQL, "lag"),
        "q9" => (Q9_SQL, "first_value"),
        "q10" => (Q10_SQL, "window"),
        _ => ("select 1 from cats", "")
    }
}

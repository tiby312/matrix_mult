#![feature(ptr_internals)]

extern crate rayon;

pub struct MatrixOwn<T>{
	pub rows:usize,
	pub columns:usize,
	pub values:Vec<T>
}

impl<T> MatrixOwn<T>{
	pub fn from_ref(a:&MatrixRef<T>)->MatrixOwn<T>{
		unimplemented!();
	}

	pub fn add_by(&mut self,a:&MatrixRef<T>){
		unimplemented!();
	}
	pub fn sub_by(&mut self,a:&MatrixRef<T>){
		unimplemented!();
	}
}
impl<T> std::ops::DerefMut for MatrixOwn<T> {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!();
    }
}

impl<T> std::ops::Deref for MatrixOwn<T> {
    type Target = MatrixRef<T>;

    fn deref(&self) -> &MatrixRef<T> {
        unimplemented!();
    }
}




pub struct MatrixRef<T> {
    pub rows: usize,
    pub columns: usize,
    pub values: std::ptr::Unique<[T]>,
}


impl<T:Copy> MatrixRef<T>{
	pub fn set_to(&mut self,other:&MatrixRef<T>){
		unimplemented!()
	}
	pub fn add_by(&mut self,other:&MatrixRef<T>){
		unimplemented!()
	}
	pub fn sub_by(&mut self,other:&MatrixRef<T>){
		unimplemented!()
	}
	pub fn split_into_four_mut<'b>(&'b mut self,point:[usize;2])->[[&'b mut MatrixRef<T>;2];2]{
		unimplemented!();
	}
	pub fn split_into_four<'b>(&'b self,point:[usize;2])->[[&'b MatrixRef<T>;2];2]{
		unimplemented!();
	}
}




pub fn mult<T:Copy+Send+Sync>(a:&mut MatrixRef<T>,b:&MatrixRef<T>){

	let midx=a.rows/2;

	if midx==0{
		return;
	}


	let mut ra=a.split_into_four_mut([midx,midx]);

	let mut rb=b.split_into_four([midx,midx]);
	
	let ((m1,m2,m3,m4),(m5,m6,m7))={
		let func1=||{
			let m1={
				
				let mut a=MatrixOwn::from_ref(ra[0][0]);
				a.add_by(ra[1][1]);

				let mut b=MatrixOwn::from_ref(rb[0][0]);
				b.add_by(rb[1][1]);


				mult(&mut a,&b);
				a
			};

			let m2={
				
				let mut a=MatrixOwn::from_ref(ra[1][0]);
				a.add_by(ra[1][1]);

				let mut b=MatrixOwn::from_ref(rb[0][0]);
				
				mult(&mut a,&b);
				a
			};

			let m3={
				
				let mut a=MatrixOwn::from_ref(ra[0][0]);
				

				let mut b=MatrixOwn::from_ref(rb[0][1]);
				b.sub_by(rb[1][1]);


				mult(&mut a,&b);
				a
			};



			let m4={
				
				let mut a=MatrixOwn::from_ref(ra[1][1]);
				

				let mut b=MatrixOwn::from_ref(rb[1][0]);
				b.sub_by(rb[0][0]);


				mult(&mut a,&b);
				a
			};
			(m1,m2,m3,m4)
		};	

		let func2=||{
			let m5={
				
				let mut a=MatrixOwn::from_ref(ra[0][0]);
				a.add_by(ra[0][1]);

				let mut b=MatrixOwn::from_ref(rb[1][1]);

				mult(&mut a,&b);
				a
			};

			let m6={
				
				let mut a=MatrixOwn::from_ref(ra[1][0]);
				a.sub_by(ra[0][0]);

				let mut b=MatrixOwn::from_ref(rb[0][0]);
				b.add_by(rb[0][1]);

				mult(&mut a,&b);
				a
			};
			

			let m7={
				
				let mut a=MatrixOwn::from_ref(ra[0][1]);
				a.sub_by(ra[1][1]);

				let mut b=MatrixOwn::from_ref(rb[1][0]);
				b.add_by(rb[1][1]);
				
				mult(&mut a,&b);
				a
			};
			(m5,m6,m7)
		};

		rayon::join(func1,func2)
	};


	ra[0][0].set_to(&m1);
	ra[0][0].add_by(&m4);
	ra[0][0].sub_by(&m5);
	ra[0][0].add_by(&m7);

	ra[0][1].set_to(&m3);
	ra[0][1].add_by(&m5);

	ra[1][0].set_to(&m2);
	ra[1][0].add_by(&m5);

	ra[1][1].set_to(&m1);
	ra[1][1].sub_by(&m2);
	ra[1][1].add_by(&m3);
	ra[1][1].add_by(&m6);

}

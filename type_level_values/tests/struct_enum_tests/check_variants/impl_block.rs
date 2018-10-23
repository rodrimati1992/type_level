pub struct UnparsedImplBlock<'a>{
    /// Each Cow may be multiple attributes each
    attributes:Vec<Cow<'a,str>>,
    attributes_exhaus:Exhaustiveness,

    where_preds:Vec<Cow<'a,str>>,
    where_preds_exhaus:Exhaustiveness,

    trait_:Cow<'a,str>,
    self_ty:Cow<'a,str>,
}

pub struct ImplBlock<'a>{
    pub(crate) attributes:HashSet<Attribute>,
    pub(crate) attributes_exhaus:Exhaustiveness,
    
    pub(crate) where_preds:HashSet<WherePredicate>,
    pub(crate) where_preds_exhaus:Exhaustiveness,
    
    pub(crate) trait_:SynPath,
    pub(crate) self_ty:SynType,
    _priv:(),
}



impl<'a> UnparsedImplBlock<'a>{
    fn priv_default()->Self{
        Self{
            attributes:Vec::new(),
            attributes_exhaus:Inexhaustive,
            where_preds:Vec::new(),
            where_preds_exhaus:Inexhaustive,
            trait_:"".into(),
            self_ty:"".into(),
        }
    }


    pub fn new<S0,S1>(trait_:S0,self_ty:S1)->Self
    where 
        S0:Into<Cow<'a,str>>,
        S1:Into<Cow<'a,str>>,
    {
        Self{
            trait_ :trait_.into(),
            self_ty:self_ty.into(),
            ..Self::priv_default()
        }
    }

    pub fn set_exhaustive(&mut self){
        self.attributes_exhaus=Exhaustive;
        self.where_preds_exhaus=Exhaustive;
    }

    pub fn add_where_pred<S>(&mut self,where_pred:S)
    where S:Into<Cow<'a,str>>,
    {
        self.where_preds.push(where_pred.into());
    }

    pub fn add_attributes<S>(&mut self,attrs:S)
    where S:Into<Cow<'a,str>>,
    {
        self.attributes.push(attrs.into());
    }

    pub fn parse(self)->ImplBlock<'a>{
        ImplBlock{
            attributes:self.attributes.iter().map(|x| parse_syn_attributes(x) ).collect(),
            attributes_exhaus:self.attributes_exhaus,
            where_preds:self.attributes.iter().map(|x| parse_where_pred(x) ).collect(),
            where_preds_exhaus:self.where_preds_exhaus,
            trait_:parse_syn_path(&*self.trait_),
            self_ty:parse_type(&*self.trait_),

            _priv:(),
        }
    }
}




impl<'a> ImplBlock<'a>{

}
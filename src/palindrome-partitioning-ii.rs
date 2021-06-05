fn main() {
    assert_eq!(Solution::min_cut("aab".to_string()), 1);
    assert_eq!(Solution::min_cut("a".to_string()), 0);
    assert_eq!(Solution::min_cut("ab".to_string()), 1);
    assert_eq!(Solution::min_cut("efe".to_string()), 0);
    assert_eq!(Solution::min_cut("fiefhgdcdcgfeibggchibffahiededbbegegdfibdbfdadfbdbceaadeceeefiheibahgececggaehbdcgebaigfacifhdbecbebfhiefchaaheiichgdbheacfbhfiaffaecicbegdgeiaiccghggdfggbebdaefcagihbdhhigdgbghbahhhdagbdaefeccfiaifffcfehfcdiiieibadcedibbedgfegibefagfccahfcbegdfdhhdgfhgbchiaieehdgdabhidhfeecgfiibediiafacagigbhchcdhbaigdcedggehhgdhedaebchcafcdehcffdiagcafcgiidhdhedgaaegdchibhdaegdfdaiiidcihifbfidechicighbcbgibadbabieaafgeagfhebfaheaeeibagdfhadifafghbfihehgcgggffgbfccgafigieadfehieafaehaggeeaaaehggffccddchibegfhdfafhadgeieggiigacbfgcagigbhbhefcadafhafdiegahbhccidbeeagcgebehheebfaechceefdiafgeddhdfcadfdafbhiifigcbddahbabbeedidhaieagheihhgffbfbiacgdaifbedaegbhigghfeiahcdieghhdabdggfcgbafgibiifdeefcbegcfcdihaeacihgdchihdadifeifdgecbchgdgdcifedacfddhhbcagaicbebbiadgbddcbagbafeadhddaeebdgdebafabghcabdhdgieiahggddigefddccfccibifgbfcdccghgceigdfdbghdihechfabhbacifgbiiiihcgifhdbhfcaiefhccibebcahidachfabicbdabibiachahggffiibbgchbidfbbhfcicfafgcagaaadbacddfiigdiiffhbbehaaacidggfbhgeaghigihggfcdcidbfccahhgaffiibbhidhdacacdfebedbiacaidaachegffaiiegeabfdgdcgdacfcfhdcbfiaaifgfaciacfghagceaaebhhibbieehhcbiggabefbeigcbhbcidbfhfcgdddgdffghidbbbfbdhcgabaagddcebaechbbiegeiggbabdhgghciheabdibefdfghbfbfebidhicdhbeghebeddgfdfhefebiiebdchifbcbahaddhbfafbbcebiigadhgcfbebgbebhfddgdeehhgdegaeedfadegfeihcgeefbbagbbacbgggciehdhiggcgaaicceeaefgcehfhfdciaghcbbgdihbhecfbgffefhgiefgeiggcebgaacefidghdfdhiabgibchdicdehahbibeddegfciaeaffgbefbbeihbafbagagedgbdadfdggfeaebaidchgdbcifhahgfdcehbahhdggcdggceiabhhafghegfdiegbcadgaecdcdddfhicabdfhbdiiceiegiedecdifhbhhfhgdbhibbdgafhgdcheefdhifgddchadbdggiidhbhegbdfdidhhfbehibiaacdfbiagcbheabaaebfeaeafbgigiefeaeheabifgcfibiddadicheahgbfhbhddaheghddceedigddhchecaghdegigbegcbfgbggdgbbigegffhcfcbbebdchffhddbfhhfgegggibhafiebcfgeaeehgdgbccbfghagfdbdfcbcigbigaccecfehcffahiafgabfcaefbghccieehhhiighcfeabffggfchfdgcfhadgidabdceediefdccceidcfbfiiaidechhbhdccccaigeegcaicabbifigcghcefaafaefd".to_string()), 1345);
}

// https://programmerstart.com/article/29911804969/
pub struct Solution {}
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        if n <= 1 {
            return 0;
        }

        let mut min_cut_num = vec![0; n];
        let mut is_palind = vec![vec![false; n]; n];
        for i in 0..n {
            is_palind[i][i] = true;
        }

        for l in 2..=n {
            for i in 0..n-l+1 {
                let j = i + l -1;

                if l == 2 {
                    is_palind[i][j] = s.as_str().chars().nth(j) == s.as_str().chars().nth(i);
                } else {
                    is_palind[i][j] = (s.as_str().chars().nth(j) == s.as_str().chars().nth(i)) && is_palind[i+1][j-1];
                }
            }
        }

        for i in 0..n {
            if is_palind[0][i] {
                min_cut_num[i] = 0;
            } else {
                min_cut_num[i] = std::i32::MAX;
                for j in 0..i {
                    if is_palind[j+1][i] && (1 + min_cut_num[j]) < min_cut_num[i] {
                        min_cut_num[i] = 1 + min_cut_num[j];
                    }
                }
            }
        }
        return min_cut_num[n - 1] as i32;
    }
}

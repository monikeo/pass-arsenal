use pass_arsenal::Hash::Whirlpool::*;

#[test]
fn test_whirlpool() {
    let plain_text = "ILoveYou";
    let cipher = whirlpool(plain_text);
    assert_eq!(cipher, "41d81aaed4e4529078404167bb607d3629b951375dc7686437ca5942d53d537edd76b85cbcc6219da0cde38b302cdda70a29b62990ef09866e4a0d3f33905fd7");
}

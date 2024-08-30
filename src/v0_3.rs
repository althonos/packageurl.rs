//! A compatibility layer for the 0.3.x version

use std::convert::TryFrom;
use std::str::FromStr;

impl<'a> TryFrom<packageurl_0_3::PackageUrl<'a>> for crate::PackageUrl<'a> {
    type Error = crate::Error;

    fn try_from(value: packageurl_0_3::PackageUrl<'a>) -> Result<Self, Self::Error> {
        crate::PackageUrl::from_str(&value.to_string())
    }
}

impl<'a> TryFrom<crate::PackageUrl<'a>> for packageurl_0_3::PackageUrl<'a> {
    type Error = packageurl_0_3::Error;

    fn try_from(value: crate::PackageUrl<'a>) -> Result<Self, Self::Error> {
        let crate::PackageUrl {
            ty,
            namespace,
            name,
            version,
            qualifiers,
            subpath,
        } = value;

        let mut result = packageurl_0_3::PackageUrl::new(ty, name)?;
        if let Some(namespace) = namespace {
            result.with_namespace(namespace);
        }
        if let Some(version) = version {
            result.with_version(version);
        }
        for (k, v) in qualifiers {
            result.add_qualifier(k, v)?;
        }
        if let Some(subpath) = subpath {
            result.with_subpath(subpath)?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use std::convert::TryInto;
    use std::str::FromStr;

    #[rstest]
    #[case("pkg:bitbucket/birkenfeld/pygments-main@244fd47e07d1014f0aed9c")]
    #[case("pkg:deb/debian/curl@7.50.3-1?arch=i386&distro=jessie")]
    #[case("pkg:docker/cassandra@sha256:244fd47e07d1004f0aed9c")]
    #[case("pkg:docker/customer/dockerimage@sha256:244fd47e07d1004f0aed9c?repository_url=gcr.io")]
    #[case("pkg:gem/jruby-launcher@1.1.2?platform=java")]
    #[case("pkg:gem/ruby-advisory-db-check@0.12.4")]
    #[case("pkg:github/package-url/purl-spec@244fd47e07d1004f0aed9c")]
    #[case("pkg:golang/google.golang.org/genproto#googleapis/api/annotations")]
    #[case("pkg:maven/org.apache.xmlgraphics/batik-anim@1.9.1?packaging=sources")]
    #[case(
        "pkg:maven/org.apache.xmlgraphics/batik-anim@1.9.1?repository_url=repo.spring.io%2Frelease"
    )]
    #[case("pkg:npm/%40angular/animation@12.3.1")]
    #[case("pkg:npm/foobar@12.3.1")]
    #[case("pkg:nuget/EnterpriseLibrary.Common@6.0.1304")]
    #[case("pkg:pypi/django@1.11.1")]
    #[case("pkg:rpm/fedora/curl@7.50.3-1.fc25?arch=i386&distro=fedora-25")]
    #[case("pkg:rpm/opensuse/curl@7.56.1-1.1.?arch=i386&distro=opensuse-tumbleweed")]
    fn roundtrip(#[case] input: &str) {
        // input
        let purl = crate::PackageUrl::from_str(input).expect("must parse");

        // convert to 0.3

        let purl_0_3: packageurl_0_3::PackageUrl = purl.clone().try_into().expect("must convert");

        // convert back to 0.4

        let purl_0_4: crate::PackageUrl = purl_0_3.try_into().expect("must convert");

        // and expect it to be the same

        assert_eq!(purl, purl_0_4);
    }
}

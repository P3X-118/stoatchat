# Changelog

## [0.12.0](https://github.com/P3X-118/stoatchat/compare/v0.11.5...v0.12.0) (2026-04-13)


### Features

* add `reason` to ServerMemberLeave event ([e17af1c](https://github.com/P3X-118/stoatchat/commit/e17af1c06471e91fdcf619383848c5f739d77939)), closes [#314](https://github.com/P3X-118/stoatchat/issues/314)
* add `reason` to ServerMemberLeave event ([e290d16](https://github.com/P3X-118/stoatchat/commit/e290d168acde6ac28a951bc951b356586a615148)), closes [#314](https://github.com/P3X-118/stoatchat/issues/314)
* add bug report template for issue tracking ([#627](https://github.com/P3X-118/stoatchat/issues/627)) ([f777e28](https://github.com/P3X-118/stoatchat/commit/f777e2863c6ca50057c8b5d0a5be14915d287724))
* add company information to email footers ([01490f5](https://github.com/P3X-118/stoatchat/commit/01490f572358a9ea81e2cb3ff29bfa57f3321ee7))
* add crond container definitions ([eb5f5f9](https://github.com/P3X-118/stoatchat/commit/eb5f5f91cdd2eb37b4b99ed66b90887b8c6f4895))
* add id field to role ([#470](https://github.com/P3X-118/stoatchat/issues/470)) ([2afea56](https://github.com/P3X-118/stoatchat/commit/2afea56e56017f02de98e67316b4457568ad5b26))
* Add Mass Mentions to the backend ([#394](https://github.com/P3X-118/stoatchat/issues/394)) ([2540860](https://github.com/P3X-118/stoatchat/commit/2540860129a2ef51b076a89c273269fb4415e334))
* add option to send message with missing replies ([18f0646](https://github.com/P3X-118/stoatchat/commit/18f06467bb3cf9ed9816d625ad55a28dc57b11f1))
* add option to send message with missing replies ([a7727bb](https://github.com/P3X-118/stoatchat/commit/a7727bba5c624766f8f0a5132b24c45e118105f9))
* add ratelimits to gifbox ([1542047](https://github.com/P3X-118/stoatchat/commit/154204742d21cbeff6e2577b00f50b495ea44631))
* add roles migration ([1b2c7b2](https://github.com/P3X-118/stoatchat/commit/1b2c7b2fa1b9873e3ffdfd305413826356ae95ab))
* add support for path style buckets ([ab58177](https://github.com/P3X-118/stoatchat/commit/ab58177dfae088806c1f41e3d19097de8a6dcc40))
* add use_starttls config option for SMTP ([a4a0934](https://github.com/P3X-118/stoatchat/commit/a4a093447526b14984ece6be4f2cf6d30c1fe49b))
* allow bots to manage emojis ([#407](https://github.com/P3X-118/stoatchat/issues/407)) ([8d88ea9](https://github.com/P3X-118/stoatchat/commit/8d88ea99636b8fc76394dff665d3fdbaf96ab84c))
* allow kicking members from voice channels ([#495](https://github.com/P3X-118/stoatchat/issues/495)) ([0dc5442](https://github.com/P3X-118/stoatchat/commit/0dc544249825a49c793309edee5ec1838458a6da))
* Allow restricting server creation to specific users ([#685](https://github.com/P3X-118/stoatchat/issues/685)) ([edfa97d](https://github.com/P3X-118/stoatchat/commit/edfa97db108c9c81828547f98a1db5315cb5ba4a))
* allow to set an icon when creating a group ([00bd56b](https://github.com/P3X-118/stoatchat/commit/00bd56b6878f828389e8b6341832c1f54a140e7c))
* always use test configuration during tests ([6c0210e](https://github.com/P3X-118/stoatchat/commit/6c0210e8a1dfc2730b49fa582e60e8a825bc0ae7))
* appeal to the almighty Spamhaus ([#524](https://github.com/P3X-118/stoatchat/issues/524)) ([5132270](https://github.com/P3X-118/stoatchat/commit/5132270f2edd6df25ce414daa42ed1b2aa6fa7a9))
* **autumn:** add /original redirect for files ([b62eeef](https://github.com/P3X-118/stoatchat/commit/b62eeef80c778d7acc539e550f2e462557465611))
* **autumn:** use real memory size for s3 cache eviction ([ed5ded5](https://github.com/P3X-118/stoatchat/commit/ed5ded5e4562b08547ff007b42b062e36a1656d2))
* bindings for node.js ([f6c57b2](https://github.com/P3X-118/stoatchat/commit/f6c57b23b42d4c9b3561ac82a0d69968c4c16a6e))
* block mentions from low trust users (account created &lt; 12 hours) ([8f4d6c8](https://github.com/P3X-118/stoatchat/commit/8f4d6c864bf55c6539cb6e3c7b4d50edf90395e1))
* **bonfire:** add disconnection mechanism ([2cb2061](https://github.com/P3X-118/stoatchat/commit/2cb20618da33e2a7fe4fb704c7c47ee69606c563))
* **bonfire:** don't fan out UserUpdate (server) by default ([36ecb48](https://github.com/P3X-118/stoatchat/commit/36ecb48c7b054a1c2a7c449fed9143324c7d3d52))
* **bonfire:** handle session deletion and logout events ([2cb2061](https://github.com/P3X-118/stoatchat/commit/2cb20618da33e2a7fe4fb704c7c47ee69606c563))
* **core/database, bindings/node:** suspend user ([0136896](https://github.com/P3X-118/stoatchat/commit/01368960f3a3d5380a373ff98baa0c54b1c7dfd1))
* **core/database:** axum implementation for User ([1a6a8a8](https://github.com/P3X-118/stoatchat/commit/1a6a8a809b3df0e02eb3e943ca5bcbbdd3d0743a))
* **core/database:** clear expired FCM tokens ([acbc1b8](https://github.com/P3X-118/stoatchat/commit/acbc1b8956d68c218f5b7cbd22ef9d55c46699a1))
* **core/database:** implement reports & snapshots ([6b488f3](https://github.com/P3X-118/stoatchat/commit/6b488f347e8b8c3fccc11793124cab52e12b5a62))
* **core/events:** add user settings / unreads to Ready payload ([412f4a9](https://github.com/P3X-118/stoatchat/commit/412f4a99d7a98e8d5995a99bbb1ad3f8b4b1f82c))
* **core/files:** support for jixel decoding ([efa7ba7](https://github.com/P3X-118/stoatchat/commit/efa7ba78edc568c1d1db48df6c4196db39eac216)), closes [#342](https://github.com/P3X-118/stoatchat/issues/342)
* **core/files:** SVG rendering for thumbnails ([a8db1cb](https://github.com/P3X-118/stoatchat/commit/a8db1cb40d7ac2a4281b0202500b01b3bf22266f))
* **core/result:** implement std::error::Error for Error ([b12e728](https://github.com/P3X-118/stoatchat/commit/b12e7285148ff4a2bf58c8ea8a4ce90231f6b861))
* **core:** add a limit to no. of outgoing pending friend requests ([8066684](https://github.com/P3X-118/stoatchat/commit/80666848cc91824d82aae78f0e7ca92e09d1e4e5))
* **core:** include user and member on Message events ([1d5dae4](https://github.com/P3X-118/stoatchat/commit/1d5dae47518261f6f904bab1bccbf1c77a3622af))
* **core:** provide user profile where appropriate ([cb97004](https://github.com/P3X-118/stoatchat/commit/cb97004d3f6dae2afba4767e178ffbb730f62734))
* **core:** separate limits for new user accounts ([6ec8007](https://github.com/P3X-118/stoatchat/commit/6ec8007e4ee9a98e553c1f5c2721bb2d3ee3df11))
* **core:** trigger logout on bot token reset ([2cb2061](https://github.com/P3X-118/stoatchat/commit/2cb20618da33e2a7fe4fb704c7c47ee69606c563))
* **core:** validation for files in reference db ([7132877](https://github.com/P3X-118/stoatchat/commit/7132877201b0ec2b8e16bbd1175f193c58bdc844))
* Detect animation in image files for fetch_preview ([#574](https://github.com/P3X-118/stoatchat/issues/574)) ([3fa0abf](https://github.com/P3X-118/stoatchat/commit/3fa0abf47f5f42ddd8ee041fe4c44fbc5ba800c1))
* env variable to not clear presences on bonfire boot ([49f7f95](https://github.com/P3X-118/stoatchat/commit/49f7f9549c92423fc21385d483974ff4de2776a9))
* expose global and user limits in root API response ([#644](https://github.com/P3X-118/stoatchat/issues/644)) ([0b522eb](https://github.com/P3X-118/stoatchat/commit/0b522ebddc17f2e3f792ff5e2347793e9849fa23))
* file deletion implementation ([249749e](https://github.com/P3X-118/stoatchat/commit/249749e14dbd2b565fbc728aa5131cd6a923be99))
* implement apple push notifications ([9ea2bd9](https://github.com/P3X-118/stoatchat/commit/9ea2bd9f2fd2002361c5a74d4dd0ef2559d90bb1))
* include groups and dms in fetch mutuals ([caa8607](https://github.com/P3X-118/stoatchat/commit/caa86074680d46223cebc20f41e9c91c41ec825d))
* include member payload in ServerMemberJoin event ([480f210](https://github.com/P3X-118/stoatchat/commit/480f210ce85271e13d1dac58a5dae08de108579d))
* include user in responses when creating and editing bots ([d6bcb84](https://github.com/P3X-118/stoatchat/commit/d6bcb844dbae30a1e06cda763091eb289da98819))
* init crond crate ([acc4317](https://github.com/P3X-118/stoatchat/commit/acc4317246ad0410f1f7e3b274d5e1549d940ff8))
* initial bulk role reorder route ([c526095](https://github.com/P3X-118/stoatchat/commit/c526095d4f0c5c365a848ac546a4ae8e00262f71))
* initial work on tenor gif searching ([b0c977b](https://github.com/P3X-118/stoatchat/commit/b0c977b324b8144c1152589546eb8fec5954c3e7))
* **january:** use real memory size for proxy cache eviction ([ed5ded5](https://github.com/P3X-118/stoatchat/commit/ed5ded5e4562b08547ff007b42b062e36a1656d2))
* load config from env vars ([#576](https://github.com/P3X-118/stoatchat/issues/576)) ([5191bd1](https://github.com/P3X-118/stoatchat/commit/5191bd16b2a905b8409838e34eb0baca96f08580))
* make message lexer use unowned string ([1561481](https://github.com/P3X-118/stoatchat/commit/1561481eb4cdc0f385fbf0a81e4950408050e11f))
* member fetching with roles ([#299](https://github.com/P3X-118/stoatchat/issues/299)) ([41ef76f](https://github.com/P3X-118/stoatchat/commit/41ef76ff3e8b46569f96668469166b20aaac3fe8))
* Message pinning ([389ecc0](https://github.com/P3X-118/stoatchat/commit/389ecc0e5cafc429ce2837dafa8eb8d23797f134))
* new parser for mention everything ([#411](https://github.com/P3X-118/stoatchat/issues/411)) ([f75d635](https://github.com/P3X-118/stoatchat/commit/f75d635c81de9d6c5e614c4cbeac229d82766e14))
* policy changes API ([c4728c6](https://github.com/P3X-118/stoatchat/commit/c4728c696d1fd2ea39c41961db751ea9360ed4f9))
* Push notification server ([#387](https://github.com/P3X-118/stoatchat/issues/387)) ([b55765d](https://github.com/P3X-118/stoatchat/commit/b55765d7c73c74c730bec4fe3f4054bd696dbbff))
* ready payload field customisation ([db57706](https://github.com/P3X-118/stoatchat/commit/db577067948f13e830b5fb773034e9713a1abaff))
* repository architecture for files crate w. added tests ([#498](https://github.com/P3X-118/stoatchat/issues/498)) ([01ded20](https://github.com/P3X-118/stoatchat/commit/01ded209c62208fc906d6aab9b08c04e860e10ef))
* require auth for search ([b5cd5e3](https://github.com/P3X-118/stoatchat/commit/b5cd5e30ef7d5e56e8964fb7c543965fa6bf5a4a))
* scaffold services/january ([c1b92ef](https://github.com/P3X-118/stoatchat/commit/c1b92ef56eb303a7f29993df8d9f44f341e18d9b))
* Send badge updates from message acks ([32d1d5d](https://github.com/P3X-118/stoatchat/commit/32d1d5df2e32389f857b38534186f6348f589ddb))
* **services/autumn:** add accept-language header ([530d68f](https://github.com/P3X-118/stoatchat/commit/530d68fe8957b3be89c35d1bd2433e49ae7f1eea)), closes [#358](https://github.com/P3X-118/stoatchat/issues/358)
* **services/autumn:** authenticate the user on upload ([6209bc7](https://github.com/P3X-118/stoatchat/commit/6209bc7152ae014154f96815b1453dcff2ce633a))
* **services/autumn:** ClamAV support ([ace6c30](https://github.com/P3X-118/stoatchat/commit/ace6c30ba593665a9943ec520df73b397fb3340f))
* **services/autumn:** cors support ([916f47e](https://github.com/P3X-118/stoatchat/commit/916f47e2f557ceac4ae79fc3bf54578af3d20039))
* **services/autumn:** deduplicate uploads ([ace6c30](https://github.com/P3X-118/stoatchat/commit/ace6c30ba593665a9943ec520df73b397fb3340f))
* **services/autumn:** download and preview files ([ebbbb5e](https://github.com/P3X-118/stoatchat/commit/ebbbb5e174f96066b95859aa1950463e746adf29))
* **services/autumn:** file size limits on upload ([6209bc7](https://github.com/P3X-118/stoatchat/commit/6209bc7152ae014154f96815b1453dcff2ce633a))
* **services/autumn:** file uploads ([ace6c30](https://github.com/P3X-118/stoatchat/commit/ace6c30ba593665a9943ec520df73b397fb3340f))
* **services/autumn:** hash files and log information ([70bdeec](https://github.com/P3X-118/stoatchat/commit/70bdeecf7736d92a10fa6a61e20af2a386547d75))
* **services/autumn:** scaffold implementation of api ([1938ebc](https://github.com/P3X-118/stoatchat/commit/1938ebc3faa1b985e8b89c5f6ca168118c7a6eda))
* **services/autumn:** strip exif data for images/videos ([70bdeec](https://github.com/P3X-118/stoatchat/commit/70bdeecf7736d92a10fa6a61e20af2a386547d75))
* **services/autumn:** work towards upload API ([24dc96f](https://github.com/P3X-118/stoatchat/commit/24dc96f80fd3421fcb04ee35c1162888588f759b))
* **services/january:** image/video embeds ([66c84e0](https://github.com/P3X-118/stoatchat/commit/66c84e0ad91a347d018a2a16aca5882d177bf942))
* **services/january:** proxy images/videos ([21335b3](https://github.com/P3X-118/stoatchat/commit/21335b329721557e253dd4a722be28208d4dc7fe))
* **services/january:** website embed generation ([ed78b25](https://github.com/P3X-118/stoatchat/commit/ed78b253ffb3173c2efd5b5444b06e7ee9ac0a4d))
* silent messages and message flags ([64a07d0](https://github.com/P3X-118/stoatchat/commit/64a07d09f433227da70e84e6a4131563f734d17b))
* trending and categories routes ([5885e06](https://github.com/P3X-118/stoatchat/commit/5885e067a627b8fff1c8ce2bf9e852ff8cf3f07a))
* version bump everything ([75a07a8](https://github.com/P3X-118/stoatchat/commit/75a07a84c0986ce75ded5739cf4d578ce11a3f1a))
* voice chats v2 ([#414](https://github.com/P3X-118/stoatchat/issues/414)) ([d567155](https://github.com/P3X-118/stoatchat/commit/d567155f124e4da74115b1a8f810062f7c6559d9))


### Bug Fixes

* Actually create crond container in github workflow ([e708c61](https://github.com/P3X-118/stoatchat/commit/e708c612cf18908d715d6897803bc4c810c0e8aa))
* add `DisplayName` back to user fields (prevented removing name) ([ea6ba59](https://github.com/P3X-118/stoatchat/commit/ea6ba598411a15eb54f0a350df077c4f75ccb279))
* add Authifier migration for last_seen ([01e0f9e](https://github.com/P3X-118/stoatchat/commit/01e0f9e5587eb193993917111d3a5e210a9f5fdd))
* add back missing early adopter badge ([a1b0e47](https://github.com/P3X-118/stoatchat/commit/a1b0e4767ad88bb3113f3bdc15cb16b224b64da7))
* add elevation check to role edit route ([aded2e3](https://github.com/P3X-118/stoatchat/commit/aded2e32396cf62611a6387f5ff9de0331a0e694))
* add emojis to ServerCreate event ([db63ac4](https://github.com/P3X-118/stoatchat/commit/db63ac406495cf57e1978aae5f0cf75623026817))
* add license to revolt-parser ([5335124](https://github.com/P3X-118/stoatchat/commit/53351243064cac8d499dd74284be73928fa78a43))
* add masquerade permission to default direct message settings ([#665](https://github.com/P3X-118/stoatchat/issues/665)) ([ab52569](https://github.com/P3X-118/stoatchat/commit/ab525699bd6663333f0e9fed6d2455e482e6a09f))
* add new crates to docker build ([5c8ece0](https://github.com/P3X-118/stoatchat/commit/5c8ece07276dfa3a7ccd1147e2dcad77ac819c37))
* add separate config option for redis events replica url ([#590](https://github.com/P3X-118/stoatchat/issues/590)) ([a75e4ea](https://github.com/P3X-118/stoatchat/commit/a75e4eabfc4b34aba7620c82ba77558a32d9e10a))
* add validator dependency to database ([0fc59b8](https://github.com/P3X-118/stoatchat/commit/0fc59b8e2a2e01813b050d4f151d91da8ca208f4))
* allow fetching discoverable servers as invites ([962c7d6](https://github.com/P3X-118/stoatchat/commit/962c7d62c7eddbb4250671cbaea3bb973ea743d3))
* allow for disabling default features ([65fbd36](https://github.com/P3X-118/stoatchat/commit/65fbd3662462aed1333b79e59155fa6377e83fcc))
* allow message pinning and unpinning for DM-like channels ([e3e1cab](https://github.com/P3X-118/stoatchat/commit/e3e1cab02ec6779e93b8f16e044453f599e663cd))
* allow message pinning and unpinning for DM-like channels ([15dec65](https://github.com/P3X-118/stoatchat/commit/15dec656a864dfc5a6ee604e023243a466faf0a2))
* allow reacting with existing emojis when at limit ([1f593a1](https://github.com/P3X-118/stoatchat/commit/1f593a1708dc0dfbc114c8af14f1590b4a1c0799))
* allow setting port and use_tls from config ([20d398d](https://github.com/P3X-118/stoatchat/commit/20d398d02c050ec6e1f0ef8c9f764128b15d0bee)), closes [#143](https://github.com/P3X-118/stoatchat/issues/143)
* always generate new test database ([bf39f18](https://github.com/P3X-118/stoatchat/commit/bf39f18f8db9c94187bff8030c00d328300ce908))
* apple music to use original url instead of metadata url ([bfe4018](https://github.com/P3X-118/stoatchat/commit/bfe4018e436a4075bae780dd4d35a9b58315e12f))
* apply uname fix to january and autumn ([8f9015a](https://github.com/P3X-118/stoatchat/commit/8f9015a6ff181d208d9269ab8691bd417d39811a))
* authifier should not use transactions for migrations ([4f13f58](https://github.com/P3X-118/stoatchat/commit/4f13f5899ba1f827b5327ccf4ae7be87818bba16))
* **autumn:** block non-images for non-attachment tags ([b62eeef](https://github.com/P3X-118/stoatchat/commit/b62eeef80c778d7acc539e550f2e462557465611))
* bonfire needs uname ([f01794a](https://github.com/P3X-118/stoatchat/commit/f01794af933d52955e4a51d6d8a77567702fc24d))
* **bonfire:** clean up Redis connection after disconnect ([8248a4a](https://github.com/P3X-118/stoatchat/commit/8248a4a5b8b2831e0f21e72fe4687ce148a59b4d))
* **bonfire:** drop connections if Redis disconnects ([f16e72e](https://github.com/P3X-118/stoatchat/commit/f16e72e3291688db319ceb8514655c61394c2380))
* **bonfire:** ignore all Redis errors but `Canceled` ([93e05e9](https://github.com/P3X-118/stoatchat/commit/93e05e9f18e3790a227cbf0bf46bc478735967c5))
* **bonfire:** make error handling consistent ([b9ae333](https://github.com/P3X-118/stoatchat/commit/b9ae333b02df2fbb793550911a531ce358a0cdd8))
* **bonfire:** random disconnects from redis ([c451e43](https://github.com/P3X-118/stoatchat/commit/c451e431cdbc5d88b28abe5f55ac2b4b91e451ae))
* **bonfire:** Ready event should include user with online status true (if applicable) ([c5f4b94](https://github.com/P3X-118/stoatchat/commit/c5f4b94aa5ca4e313aa25cb49af87570843c4949))
* **bonfire:** send InvalidSession error if token is not sent successfully ([da9a91e](https://github.com/P3X-118/stoatchat/commit/da9a91e05f739a3330bff1d443ba4c55182126b8)), closes [#289](https://github.com/P3X-118/stoatchat/issues/289)
* **bonfire:** use fred for redis ([88c2232](https://github.com/P3X-118/stoatchat/commit/88c2232a6c74ee415da17ddf4aed68c0ca6480cb))
* **bonfire:** use REDIS_URI env var ([3081933](https://github.com/P3X-118/stoatchat/commit/308193377d08454fac3381513957ff011ae586a3))
* bot model test did not destruct tuple ([f9d9059](https://github.com/P3X-118/stoatchat/commit/f9d9059e7346801ca8a4cbe2e2d3bc6fa48d85e0))
* bots in multiple voice channel logic ([#544](https://github.com/P3X-118/stoatchat/issues/544)) ([94cb916](https://github.com/P3X-118/stoatchat/commit/94cb916231b9b8befb2e94065917ff40815bec52))
* Bump revolt_a2 dep ([39230c5](https://github.com/P3X-118/stoatchat/commit/39230c559ad0884e94a5f85b8d3418aae3e5a85e))
* change permission check for fetching channel webhooks ([e3723d6](https://github.com/P3X-118/stoatchat/commit/e3723d647effb81ea3d3919d848faf64dbe89829))
* Check for appropriate permission for removing other users avatar ([#657](https://github.com/P3X-118/stoatchat/issues/657)) ([d56135e](https://github.com/P3X-118/stoatchat/commit/d56135e0cbc713884c9378832952f7ad490fa315))
* check server ownership for text channel ([456bf7b](https://github.com/P3X-118/stoatchat/commit/456bf7b42abc768a10e437eab92d23903f6b7dce))
* checkout repo. before bumping lock ([#490](https://github.com/P3X-118/stoatchat/issues/490)) ([b2da2a8](https://github.com/P3X-118/stoatchat/commit/b2da2a858787853be43136fd526a0bd72baf78ef))
* **ci:** build API then run in background to prevent timeout ([4c4dada](https://github.com/P3X-118/stoatchat/commit/4c4dada3daf7bdc425bf8d62fe5d739500a24176))
* **ci:** only start run command in background ([6d4f772](https://github.com/P3X-118/stoatchat/commit/6d4f772d787109d3f7c2dddc14975fc04ec6a0e4))
* **ci:** pipeline fixes (marked as fix to force release) ([#483](https://github.com/P3X-118/stoatchat/issues/483)) ([303e52b](https://github.com/P3X-118/stoatchat/commit/303e52b476585eea81c33837f1b01506ce387684))
* **ci:** publish images under stoatchat and remove docker hub ([d65c1a1](https://github.com/P3X-118/stoatchat/commit/d65c1a1ab3bdc7e5684b03f280af77d881661a3d))
* **ci:** remnants from branch rename ([78fb6e1](https://github.com/P3X-118/stoatchat/commit/78fb6e1982fd380296a75af8a51544fdc53c0ec1))
* **ci:** try to work-around runner being killed ([d7213fa](https://github.com/P3X-118/stoatchat/commit/d7213fa40940f0eb741a8466aec35562214707ba))
* **ci:** use correct port for Rust test ([b62eeef](https://github.com/P3X-118/stoatchat/commit/b62eeef80c778d7acc539e550f2e462557465611))
* configure new authifier properties ([3a7ebad](https://github.com/P3X-118/stoatchat/commit/3a7ebad8838f088048b0531283de7321ce7f22f2))
* continue fix ([878c1a8](https://github.com/P3X-118/stoatchat/commit/878c1a83ad9ef6517d7664220890116e2bd81faa))
* **core/database:** fetch users mentioned in system messages ([4055363](https://github.com/P3X-118/stoatchat/commit/4055363cff8a892b983ba7afde3245e88a2b41ce))
* **core/database:** include users from pin events ([535f016](https://github.com/P3X-118/stoatchat/commit/535f01604bee5d0635654c7c8b5b236ef1ce28d9))
* **core/database:** only run outgoing friend checks if we are creating new request ([de5add0](https://github.com/P3X-118/stoatchat/commit/de5add09d05f95721672678c4c5c2f0512700f32)), closes [#327](https://github.com/P3X-118/stoatchat/issues/327)
* **core/database:** set do not set online status if presence is Invisible ([5069686](https://github.com/P3X-118/stoatchat/commit/506968634ed6449fcf32110837a2088bc33369d9)), closes [#34](https://github.com/P3X-118/stoatchat/issues/34)
* **core/database:** skip insertion / deletion if no invites need to be corrected ([a5d0cdf](https://github.com/P3X-118/stoatchat/commit/a5d0cdf0dddaca45877a76ab335382c59374c9ae))
* **core/database:** store member during permission query ([21ffea0](https://github.com/P3X-118/stoatchat/commit/21ffea0f9f05ee4116878206e59b1e67c37bd032))
* **core/models:** use IndexMap for reactions ([226dbca](https://github.com/P3X-118/stoatchat/commit/226dbca6e05407a863ccaf62ec98229c9a1fd64a))
* **core/models:** validator feature flag not properly gatekept ([96d9021](https://github.com/P3X-118/stoatchat/commit/96d90215d23487cde36e50646ebf7b6e62172351))
* **core:** fix _id typo ([#384](https://github.com/P3X-118/stoatchat/issues/384)) ([443f374](https://github.com/P3X-118/stoatchat/commit/443f374f2318dcb44f4c0a7cc1994d7c784e2bac))
* **core:** publish user settings event ([4d42fb7](https://github.com/P3X-118/stoatchat/commit/4d42fb74e7a843b62c52ed5f28023e816ca92dce))
* correct miniz_oxide in lockfile ([#478](https://github.com/P3X-118/stoatchat/issues/478)) ([5d27a91](https://github.com/P3X-118/stoatchat/commit/5d27a91e901dd2ea3e860aeaed8468db6c5f3214))
* correct shebang for try-tag-and-release ([050ba16](https://github.com/P3X-118/stoatchat/commit/050ba16d4adad5d0fb247867aa3e94e3d42bd12d))
* correct string_cache in lockfile ([#479](https://github.com/P3X-118/stoatchat/issues/479)) ([0b178fc](https://github.com/P3X-118/stoatchat/commit/0b178fc791583064bf9ca94b1d39b42d021e1d79))
* cut events traffic while we engineer a new events architecture ([#559](https://github.com/P3X-118/stoatchat/issues/559)) ([a11986b](https://github.com/P3X-118/stoatchat/commit/a11986ba1ad16b672ff1080913a684567d88adbb))
* cut presence traffic too while we engineer a new events architecture ([#561](https://github.com/P3X-118/stoatchat/issues/561)) ([1f8ea96](https://github.com/P3X-118/stoatchat/commit/1f8ea963ad742f693f405e6438f1c343c81e6579))
* **database:** fetch all server emojis instead of one ([25016ef](https://github.com/P3X-118/stoatchat/commit/25016efbc2bbe96fbe79f2dc1b29dee3c924007c))
* **database:** set channel ids for new server object ([aed8f69](https://github.com/P3X-118/stoatchat/commit/aed8f69f34e59eba850908d543e14d2ebfe2e220))
* db migration for webhooks does not consider missing channels ([705e517](https://github.com/P3X-118/stoatchat/commit/705e517871af5ed3308af33dd6164685e623b259)), closes [#378](https://github.com/P3X-118/stoatchat/issues/378)
* default video resolution is a non-existent size ([#601](https://github.com/P3X-118/stoatchat/issues/601)) ([0698e11](https://github.com/P3X-118/stoatchat/commit/0698e115e8d003d615e468c4fb9654e6bbc9107f)), closes [#588](https://github.com/P3X-118/stoatchat/issues/588)
* delete associated server objects ([acb4190](https://github.com/P3X-118/stoatchat/commit/acb4190f70f990c73a252769658808f7fdd7f18a))
* delete attachments by `used_for.id` and add corresponding index ([cbf9e81](https://github.com/P3X-118/stoatchat/commit/cbf9e81256a05160f26e8bd27213326f9073f9a5))
* **delta:** add check to roles_fetch route ([f8ec6ba](https://github.com/P3X-118/stoatchat/commit/f8ec6ba5ffc9f195e67e0a8d1163deb9e25b0cd0))
* **delta:** don't specify the other member in permission check ([d179d1e](https://github.com/P3X-118/stoatchat/commit/d179d1e6957e924f1b1ae6a633d05f72b5c8f1cc))
* **delta:** MemberResponse only include roles member has ([e4b5a23](https://github.com/P3X-118/stoatchat/commit/e4b5a23f71120074939ab0404db0206326a228a2))
* **delta:** set ratelimit headers on 429 responses ([ea00f0f](https://github.com/P3X-118/stoatchat/commit/ea00f0fec1521eecf20e371e2f3f0e63801799ca))
* **delta:** use untagged enum for MemberResponse ([9663caa](https://github.com/P3X-118/stoatchat/commit/9663caa1ffd708698d10a8cea5221f432a563e5c))
* detach rather than delete emoji during server deletion ([12ae781](https://github.com/P3X-118/stoatchat/commit/12ae781621616c2b41af6bbe6e26f05e02ad3557))
* disable publish for services ([#485](https://github.com/P3X-118/stoatchat/issues/485)) ([d13609c](https://github.com/P3X-118/stoatchat/commit/d13609c37279d6a40445dcd99564e5c3dd03bac1))
* do not handle error early ([2cb12a3](https://github.com/P3X-118/stoatchat/commit/2cb12a3d59c0b4ee564edd515725693f97b5ec9c))
* **docs:** Update GitHub links ([#647](https://github.com/P3X-118/stoatchat/issues/647)) ([b830631](https://github.com/P3X-118/stoatchat/commit/b830631bd25a546844b7bdd30386084bb365e4de))
* don't allow sending typing notification to unknown channel ([96fb0ee](https://github.com/P3X-118/stoatchat/commit/96fb0eecca7143ad2fc59d9c552c035b539ecb7e)), closes [#177](https://github.com/P3X-118/stoatchat/issues/177)
* don't allow users to time themselves out ([add3f40](https://github.com/P3X-118/stoatchat/commit/add3f40b2308ef2858b5a4bc47d31dec90995abb)), closes [#376](https://github.com/P3X-118/stoatchat/issues/376)
* don't bump the lockfile version ([b177a3e](https://github.com/P3X-118/stoatchat/commit/b177a3e201281c90b4187896b997006a8043a9e2))
* don't copy the old test file ([9be171c](https://github.com/P3X-118/stoatchat/commit/9be171c7b635e3bd10170d28ec9f3bcfe9509e2c))
* don't create "previews" of GIFs ([7b15006](https://github.com/P3X-118/stoatchat/commit/7b15006a5088b3914be8589cb7214cbee9ef55e8))
* don't exceed the max emoji limit by one ([09a848f](https://github.com/P3X-118/stoatchat/commit/09a848f5a6c9599b6066a5e5a5650267276e84c4)), closes [#295](https://github.com/P3X-118/stoatchat/issues/295)
* don't remove timeouts when a member leaves a server ([#409](https://github.com/P3X-118/stoatchat/issues/409)) ([e635bc2](https://github.com/P3X-118/stoatchat/commit/e635bc23ec857d648d5705e1a3875d7bc3402b0d))
* don't try to mount env logger twice ([5362e84](https://github.com/P3X-118/stoatchat/commit/5362e847304b0d70aa9b23f8316bc225c154546c))
* don't update the same field while trying to remove it ([f4ee35f](https://github.com/P3X-118/stoatchat/commit/f4ee35fb093ca49f0a64ff4b17fd61587df28145)), closes [#392](https://github.com/P3X-118/stoatchat/issues/392)
* don't use a bitop for OR ([#676](https://github.com/P3X-118/stoatchat/issues/676)) ([5701b5c](https://github.com/P3X-118/stoatchat/commit/5701b5c18c513f796af365169ceaea372a22638c))
* don't use failed file hashes ([af0d24c](https://github.com/P3X-118/stoatchat/commit/af0d24c7c649fb9282e4730c12a6c06c204b41cf)), closes [#377](https://github.com/P3X-118/stoatchat/issues/377)
* dont leak invisible presence to others ([f4281c7](https://github.com/P3X-118/stoatchat/commit/f4281c70504bed1e0e07a5730c1c2548a089de17))
* ensure easypwned api is actually set before using ([68bcfcb](https://github.com/P3X-118/stoatchat/commit/68bcfcb6265257265d3cc2b590c53927f3fb9b3b))
* ensure limit is always at least 1 when sent to database ([5f84daa](https://github.com/P3X-118/stoatchat/commit/5f84daa9dba34c103cd83a2ee1f5e5ba900bfe94))
* ensure original order is correctly sorted on edit route ([73b576a](https://github.com/P3X-118/stoatchat/commit/73b576a75f66717a31c4f2506460db6611318614))
* ensure ratelimiter adjusts for version prefix ([5f39403](https://github.com/P3X-118/stoatchat/commit/5f39403ce78b90416305296ffa81e0137f0326db))
* ensure server ID is fanned out with role ranks update ([73b576a](https://github.com/P3X-118/stoatchat/commit/73b576a75f66717a31c4f2506460db6611318614))
* execute query to remove users from groups ([a9e3093](https://github.com/P3X-118/stoatchat/commit/a9e309395ed1159072f1725f7db59d1e312e01bc))
* execute query to remove users from groups ([78cd89e](https://github.com/P3X-118/stoatchat/commit/78cd89ec32e0caac580e9bb3e313fa4de1010e10))
* expose ratelimit headers via cors ([#496](https://github.com/P3X-118/stoatchat/issues/496)) ([a1a2125](https://github.com/P3X-118/stoatchat/commit/a1a21252d0ad58937e41f16e5fb86f96bebd2a51))
* Fix typo for p256dh in vapid notification flow ([#622](https://github.com/P3X-118/stoatchat/issues/622)) ([a80ad1c](https://github.com/P3X-118/stoatchat/commit/a80ad1cbe58b8af5e45751e51d94d93c1cea1c9f))
* fully qualify create_error in create_database_error macro ([4e7049d](https://github.com/P3X-118/stoatchat/commit/4e7049d51e321ab6e3601faae5ed41c884830d9c))
* github webhook incorrect payload and formatting ([#468](https://github.com/P3X-118/stoatchat/issues/468)) ([dc9c82a](https://github.com/P3X-118/stoatchat/commit/dc9c82aa4e9667ea6639256c65ac8de37a24d1f7))
* implement Serialize to ClientMessage ([dea0f67](https://github.com/P3X-118/stoatchat/commit/dea0f675dde7a63c7a59b38d469f878b7a8a3af4))
* import std::env in config ([13b95d3](https://github.com/P3X-118/stoatchat/commit/13b95d383eb9f4f4a5102e0ada7b4e7b4d122b61))
* include `production` default value for config ([bcf6561](https://github.com/P3X-118/stoatchat/commit/bcf6561c0c03c45081dd24b3ec100714420d0d8e))
* include `pushd` key in Revolt.toml ([be89e62](https://github.com/P3X-118/stoatchat/commit/be89e62d5411765ddd35c8fce7a5c1ccfaf3c8ce))
* include revolt-parser in build scripts ([aa8f857](https://github.com/P3X-118/stoatchat/commit/aa8f8575bcd1e2bd6a31017daa7b624f9ef2f4d9))
* include user for pinned system messages ([b45ae2c](https://github.com/P3X-118/stoatchat/commit/b45ae2cd4d32a664b9e9d5fb484c288d38d82b59))
* last message in a channel could not be acked, resulting in ghost dms and channel messages ([03f2e3b](https://github.com/P3X-118/stoatchat/commit/03f2e3b1bf36151e84261bd8670168a06dea8b75))
* local tests with overrides were missing prerequisites ([1690df9](https://github.com/P3X-118/stoatchat/commit/1690df998da85ffeb187b362bd02af127f00a5fa))
* logic error in initial data check ([99f400b](https://github.com/P3X-118/stoatchat/commit/99f400bc7bded575de1d93615710db5a16f55789))
* make docker the default connection for config ([72e3129](https://github.com/P3X-118/stoatchat/commit/72e312955745f2159bbdac975f09a9b7561bd019))
* mangled symbol ([18888ea](https://github.com/P3X-118/stoatchat/commit/18888eae5ff6d5d9d92033e266c8f7b106cc39e5))
* match Debian releases for build ([730039e](https://github.com/P3X-118/stoatchat/commit/730039eda6fa8535ae66dbe42ca27644f8b5d564))
* match new error type for status code ([defc9ec](https://github.com/P3X-118/stoatchat/commit/defc9ec79bddf7f1a72e0a8eb5d7f6639acfef73))
* must not use replace_one as it expects no change in _id ([19e72ba](https://github.com/P3X-118/stoatchat/commit/19e72babc9af62c943d0c79a90197b3f92c66fa5))
* newly created roles should be ranked the lowest ([947eb15](https://github.com/P3X-118/stoatchat/commit/947eb15771ed6785b3dcd16c354c03ded5e4cbe0))
* no node state set on channel creation ([#653](https://github.com/P3X-118/stoatchat/issues/653)) ([24d0d2b](https://github.com/P3X-118/stoatchat/commit/24d0d2b7266f6f8a692d0a52704acfecf517674c))
* permit empty `remove` array in edit requests ([6ad3da5](https://github.com/P3X-118/stoatchat/commit/6ad3da5f35f989a2e7d8e29718b98374248e76af))
* persist credentials for git repo ([#492](https://github.com/P3X-118/stoatchat/issues/492)) ([c674a9f](https://github.com/P3X-118/stoatchat/commit/c674a9fd4e0abbd51569870e4b38074d4a1de03c))
* pipeline fixes ([#487](https://github.com/P3X-118/stoatchat/issues/487)) ([aeeafeb](https://github.com/P3X-118/stoatchat/commit/aeeafebefc36a43a656cf797c9251ca50292733c))
* populate empty vector if clear field is missing ([876068a](https://github.com/P3X-118/stoatchat/commit/876068a37ed867104850130e63d50071300db056)), closes [#367](https://github.com/P3X-118/stoatchat/issues/367)
* port is wrong for maildev ([530ff4f](https://github.com/P3X-118/stoatchat/commit/530ff4f9c7fa756bd00a6bc74f7961da2df08f3e))
* preserve order of replies in message ([#447](https://github.com/P3X-118/stoatchat/issues/447)) ([657a3f0](https://github.com/P3X-118/stoatchat/commit/657a3f08e5d652814bbf0647e089ed9ebb139bbf))
* prevent potential double fetching of cached server ([b90e7a4](https://github.com/P3X-118/stoatchat/commit/b90e7a4412ea214b71ad23327a59b24766517c85))
* prevent timing out members which have TimeoutMembers permission ([e36fc97](https://github.com/P3X-118/stoatchat/commit/e36fc9738bac0de4f3fcbccba521f1e3754f7ae7))
* redis_url vs redis_uri in config ([#666](https://github.com/P3X-118/stoatchat/issues/666)) ([b0b728f](https://github.com/P3X-118/stoatchat/commit/b0b728fb0dbc9ee28360301de1c3ea501bbbff1d))
* relax settings name regex ([3a34159](https://github.com/P3X-118/stoatchat/commit/3a3415915f0d0fdce1499d47a2b7fa097f5946ea))
* remove authentication tag bytes from attachment download ([32e6600](https://github.com/P3X-118/stoatchat/commit/32e6600272b885c595c094f0bc69459250220dcb))
* remove SavedMessages and Group due to given permissions ([5eea0bb](https://github.com/P3X-118/stoatchat/commit/5eea0bbc9a009a852e8a64df68aada871ff61dc3))
* rename openapi operation ids ([6048587](https://github.com/P3X-118/stoatchat/commit/6048587d348fbca0dc3a9b47690c56df8fece576)), closes [#406](https://github.com/P3X-118/stoatchat/issues/406)
* replace some links and Revolt mentions to current Stoat ([#515](https://github.com/P3X-118/stoatchat/issues/515)) ([d629e89](https://github.com/P3X-118/stoatchat/commit/d629e89304be2f0011e189293b278f07d346aa7d))
* replace user with target user when editing someone else ([00e8817](https://github.com/P3X-118/stoatchat/commit/00e881799f255fecfb302eb08b82dd50b1de784e))
* respect `Permission::SendEmbeds` when editing a message ([c5494aa](https://github.com/P3X-118/stoatchat/commit/c5494aa1ca98fa4b60dd625f16ebf112981355fe))
* respond with 201 if no body in requests ([#465](https://github.com/P3X-118/stoatchat/issues/465)) ([24fedf8](https://github.com/P3X-118/stoatchat/commit/24fedf8c4d9cd3160bdec97aa451520f8beaa739))
* return more appropriate error on token fetch fail ([4be3bdc](https://github.com/P3X-118/stoatchat/commit/4be3bdc4c3579aefd88f2b4bc3af32564bcefad3))
* route ranking ([e00603f](https://github.com/P3X-118/stoatchat/commit/e00603f27665cb3857ba6b249bff99a0cc5a6a3d))
* send pin system messages ([96bf73c](https://github.com/P3X-118/stoatchat/commit/96bf73cb573223e90d81e9b627787667d7a5bacc))
* send push notifications for DM and group messages ([#660](https://github.com/P3X-118/stoatchat/issues/660)) ([52c0d2f](https://github.com/P3X-118/stoatchat/commit/52c0d2f266b76d8975bba2d5e75c62bb30149c45))
* **services/january:** put html parsing into block to prevent issues with futures ([bb20207](https://github.com/P3X-118/stoatchat/commit/bb202079e00455e92218035af66e719112acadbe))
* **services/january:** reddit embeds by spoofing as discord / mapping to old reddit ([bb6bcda](https://github.com/P3X-118/stoatchat/commit/bb6bcda8bd11299506807c9d4ac8fc6f0d24bbad)), closes [#360](https://github.com/P3X-118/stoatchat/issues/360)
* **services/january:** remove image if video present and hence fix logic error ([58d3c5c](https://github.com/P3X-118/stoatchat/commit/58d3c5cc2e77cf9e67ea9bc18de4c9a1a4193a8e))
* **services/january:** support svg for embed generation ([520fb02](https://github.com/P3X-118/stoatchat/commit/520fb02fb6f3f002a69789d93bde860ab0317d92))
* **services/january:** YouTube fallback ([afd8c90](https://github.com/P3X-118/stoatchat/commit/afd8c906bab23a1967a2f4eed2ca54cc5ad0b5dc))
* set relationship for users in BulkMessageResponse ([219c16a](https://github.com/P3X-118/stoatchat/commit/219c16a69ca91b44d5a5cd61ae55d750ebdc8821))
* show full branch name on github webhook messages ([d2e83c9](https://github.com/P3X-118/stoatchat/commit/d2e83c94f3f078313b9a3695507c06e675faf622))
* specify configuration ([eda3643](https://github.com/P3X-118/stoatchat/commit/eda36436a862bcab92f7ac2cf1c2dd88c41e52a4))
* spelling issue ([78757ac](https://github.com/P3X-118/stoatchat/commit/78757ac7f1a4a6774e3e47af941e060dae657d60))
* store server id in redis and in room metadata to be able to delete voice state in all scenarios ([#656](https://github.com/P3X-118/stoatchat/issues/656)) ([49c6289](https://github.com/P3X-118/stoatchat/commit/49c628958070e4f0a5edc764d3b48158589219d9))
* swap to using reqwest for query building ([38dd4d1](https://github.com/P3X-118/stoatchat/commit/38dd4d10797b3e6e397fc219e818f379bdff19f2))
* Switch to remove for unpinning ([c50435d](https://github.com/P3X-118/stoatchat/commit/c50435d499d329381a28cda03aa81b1f3f4c4a5d))
* temp fix for spam attack via notification bug abuse ([d9deadc](https://github.com/P3X-118/stoatchat/commit/d9deadc65ab5bd277586c944e66c7468a3b7eb3a))
* temp fix for spam attack via notification bug abuse, but git actually commits the changes this time ([397b987](https://github.com/P3X-118/stoatchat/commit/397b9878e1c942f9ee9e9afa81c57563900adeba))
* **tests:** add policy change value to text fixtures ([9846d8a](https://github.com/P3X-118/stoatchat/commit/9846d8aac20421af1deba24798cde98753b03acf))
* thumbnailification requires rgb8/rgba8 ([#505](https://github.com/P3X-118/stoatchat/issues/505)) ([413aa04](https://github.com/P3X-118/stoatchat/commit/413aa04dcaf8bff3935ed1e5f31432e11a03ce6f))
* uname is missing from crond ([#675](https://github.com/P3X-118/stoatchat/issues/675)) ([dc4438b](https://github.com/P3X-118/stoatchat/commit/dc4438bc3c7b2cad8d442b3cd438afb9ed566a5e))
* update `Revolt` -&gt; `Stoat` in email titles/desc. ([#508](https://github.com/P3X-118/stoatchat/issues/508)) ([84483ce](https://github.com/P3X-118/stoatchat/commit/84483cee7af3e5dfa16f7fe13e334c4d9f5abd60))
* use `trust_cloudflare` config value instead of env var ([cc7a796](https://github.com/P3X-118/stoatchat/commit/cc7a7962a882e1627fcd0bc75858a017415e8cfc))
* use a valid default permission set for groups ([d4d5b23](https://github.com/P3X-118/stoatchat/commit/d4d5b23c7d545311b4bd0a737f315ff611bc9544))
* Use correct enum tagging for invites ([471e0f5](https://github.com/P3X-118/stoatchat/commit/471e0f55e476616f96cfba73211e1453147ac09d))
* use correct id to lookup user from session ([3e26e7e](https://github.com/P3X-118/stoatchat/commit/3e26e7e89d048125480d62bc8ac6774b625a7f87))
* use MSRV for dockerfile ([79e1388](https://github.com/P3X-118/stoatchat/commit/79e138800063c724919513a51bd5110b5d0a4ea4))
* use our own result types instead of tenors types ([a92152d](https://github.com/P3X-118/stoatchat/commit/a92152d86da136997817e797c7af8e38731cdde8))
* use Rust 1.92.0 for Docker build ([#503](https://github.com/P3X-118/stoatchat/issues/503)) ([98da8a2](https://github.com/P3X-118/stoatchat/commit/98da8a28a0aa2fee4e8eee1d86bd7c49e3187477))
* validate masquerade on new message models ([9bf60e8](https://github.com/P3X-118/stoatchat/commit/9bf60e87e98ac53859d5690672e6db08103c2f92))
* wait on unwaited futures ([8099310](https://github.com/P3X-118/stoatchat/commit/8099310f89091092c0b25f2fd5e288a69238a232))
* wrong match keyword causing disconnects on typing event ([5c40f66](https://github.com/P3X-118/stoatchat/commit/5c40f66010ee0cda81249ad138928a56a3eebaf4))


### Performance Improvements

* **lto:** enable link-time optimization ([#371](https://github.com/P3X-118/stoatchat/issues/371)) ([42367f4](https://github.com/P3X-118/stoatchat/commit/42367f477c13bb4b50e2de898744b22bbac211ad))


### Reverts

* disable user update events ([#593](https://github.com/P3X-118/stoatchat/issues/593)) ([1c98ead](https://github.com/P3X-118/stoatchat/commit/1c98ead69579b4700be0b51c9020bb8402336cc6))

## [0.11.5](https://github.com/stoatchat/stoatchat/compare/v0.11.4...v0.11.5) (2026-02-17)


### Reverts

* disable user update events ([#593](https://github.com/stoatchat/stoatchat/issues/593)) ([1c98ead](https://github.com/stoatchat/stoatchat/commit/1c98ead69579b4700be0b51c9020bb8402336cc6))

## [0.11.4](https://github.com/stoatchat/stoatchat/compare/v0.11.3...v0.11.4) (2026-02-16)


### Bug Fixes

* add separate config option for redis events replica url ([#590](https://github.com/stoatchat/stoatchat/issues/590)) ([a75e4ea](https://github.com/stoatchat/stoatchat/commit/a75e4eabfc4b34aba7620c82ba77558a32d9e10a))

## [0.11.3](https://github.com/stoatchat/stoatchat/compare/v0.11.2...v0.11.3) (2026-02-13)


### Bug Fixes

* cut presence traffic too while we engineer a new events architecture ([#561](https://github.com/stoatchat/stoatchat/issues/561)) ([1f8ea96](https://github.com/stoatchat/stoatchat/commit/1f8ea963ad742f693f405e6438f1c343c81e6579))

## [0.11.2](https://github.com/stoatchat/stoatchat/compare/v0.11.1...v0.11.2) (2026-02-13)


### Bug Fixes

* cut events traffic while we engineer a new events architecture ([#559](https://github.com/stoatchat/stoatchat/issues/559)) ([a11986b](https://github.com/stoatchat/stoatchat/commit/a11986ba1ad16b672ff1080913a684567d88adbb))

## [0.11.1](https://github.com/stoatchat/stoatchat/compare/v0.11.0...v0.11.1) (2026-02-13)


### Bug Fixes

* bots in multiple voice channel logic ([#544](https://github.com/stoatchat/stoatchat/issues/544)) ([94cb916](https://github.com/stoatchat/stoatchat/commit/94cb916231b9b8befb2e94065917ff40815bec52))

## [0.11.0](https://github.com/stoatchat/stoatchat/compare/v0.10.3...v0.11.0) (2026-02-10)


### Features

* appeal to the almighty Spamhaus ([#524](https://github.com/stoatchat/stoatchat/issues/524)) ([5132270](https://github.com/stoatchat/stoatchat/commit/5132270f2edd6df25ce414daa42ed1b2aa6fa7a9))

## [0.10.3](https://github.com/stoatchat/stoatchat/compare/v0.10.2...v0.10.3) (2026-02-07)


### Bug Fixes

* update `Revolt` -&gt; `Stoat` in email titles/desc. ([#508](https://github.com/stoatchat/stoatchat/issues/508)) ([84483ce](https://github.com/stoatchat/stoatchat/commit/84483cee7af3e5dfa16f7fe13e334c4d9f5abd60))

## [0.10.2](https://github.com/stoatchat/stoatchat/compare/v0.10.1...v0.10.2) (2026-01-25)


### Bug Fixes

* thumbnailification requires rgb8/rgba8 ([#505](https://github.com/stoatchat/stoatchat/issues/505)) ([413aa04](https://github.com/stoatchat/stoatchat/commit/413aa04dcaf8bff3935ed1e5f31432e11a03ce6f))

## [0.10.1](https://github.com/stoatchat/stoatchat/compare/v0.10.0...v0.10.1) (2026-01-25)


### Bug Fixes

* use Rust 1.92.0 for Docker build ([#503](https://github.com/stoatchat/stoatchat/issues/503)) ([98da8a2](https://github.com/stoatchat/stoatchat/commit/98da8a28a0aa2fee4e8eee1d86bd7c49e3187477))

## [0.10.0](https://github.com/stoatchat/stoatchat/compare/v0.9.4...v0.10.0) (2026-01-25)


### Features

* allow kicking members from voice channels ([#495](https://github.com/stoatchat/stoatchat/issues/495)) ([0dc5442](https://github.com/stoatchat/stoatchat/commit/0dc544249825a49c793309edee5ec1838458a6da))
* repository architecture for files crate w. added tests ([#498](https://github.com/stoatchat/stoatchat/issues/498)) ([01ded20](https://github.com/stoatchat/stoatchat/commit/01ded209c62208fc906d6aab9b08c04e860e10ef))


### Bug Fixes

* expose ratelimit headers via cors ([#496](https://github.com/stoatchat/stoatchat/issues/496)) ([a1a2125](https://github.com/stoatchat/stoatchat/commit/a1a21252d0ad58937e41f16e5fb86f96bebd2a51))

## [0.9.4](https://github.com/stoatchat/stoatchat/compare/v0.9.3...v0.9.4) (2026-01-10)


### Bug Fixes

* checkout repo. before bumping lock ([#490](https://github.com/stoatchat/stoatchat/issues/490)) ([b2da2a8](https://github.com/stoatchat/stoatchat/commit/b2da2a858787853be43136fd526a0bd72baf78ef))
* persist credentials for git repo ([#492](https://github.com/stoatchat/stoatchat/issues/492)) ([c674a9f](https://github.com/stoatchat/stoatchat/commit/c674a9fd4e0abbd51569870e4b38074d4a1de03c))

## [0.9.3](https://github.com/stoatchat/stoatchat/compare/v0.9.2...v0.9.3) (2026-01-10)


### Bug Fixes

* pipeline fixes ([#487](https://github.com/stoatchat/stoatchat/issues/487)) ([aeeafeb](https://github.com/stoatchat/stoatchat/commit/aeeafebefc36a43a656cf797c9251ca50292733c))

## [0.9.2](https://github.com/stoatchat/stoatchat/compare/v0.9.1...v0.9.2) (2026-01-10)


### Bug Fixes

* disable publish for services ([#485](https://github.com/stoatchat/stoatchat/issues/485)) ([d13609c](https://github.com/stoatchat/stoatchat/commit/d13609c37279d6a40445dcd99564e5c3dd03bac1))

## [0.9.1](https://github.com/stoatchat/stoatchat/compare/v0.9.0...v0.9.1) (2026-01-10)


### Bug Fixes

* **ci:** pipeline fixes (marked as fix to force release) ([#483](https://github.com/stoatchat/stoatchat/issues/483)) ([303e52b](https://github.com/stoatchat/stoatchat/commit/303e52b476585eea81c33837f1b01506ce387684))

## [0.9.0](https://github.com/stoatchat/stoatchat/compare/v0.8.8...v0.9.0) (2026-01-10)


### Features

* add id field to role ([#470](https://github.com/stoatchat/stoatchat/issues/470)) ([2afea56](https://github.com/stoatchat/stoatchat/commit/2afea56e56017f02de98e67316b4457568ad5b26))
* add ratelimits to gifbox ([1542047](https://github.com/stoatchat/stoatchat/commit/154204742d21cbeff6e2577b00f50b495ea44631))
* include groups and dms in fetch mutuals ([caa8607](https://github.com/stoatchat/stoatchat/commit/caa86074680d46223cebc20f41e9c91c41ec825d))
* include member payload in ServerMemberJoin event ([480f210](https://github.com/stoatchat/stoatchat/commit/480f210ce85271e13d1dac58a5dae08de108579d))
* initial work on tenor gif searching ([b0c977b](https://github.com/stoatchat/stoatchat/commit/b0c977b324b8144c1152589546eb8fec5954c3e7))
* make message lexer use unowned string ([1561481](https://github.com/stoatchat/stoatchat/commit/1561481eb4cdc0f385fbf0a81e4950408050e11f))
* ready payload field customisation ([db57706](https://github.com/stoatchat/stoatchat/commit/db577067948f13e830b5fb773034e9713a1abaff))
* require auth for search ([b5cd5e3](https://github.com/stoatchat/stoatchat/commit/b5cd5e30ef7d5e56e8964fb7c543965fa6bf5a4a))
* trending and categories routes ([5885e06](https://github.com/stoatchat/stoatchat/commit/5885e067a627b8fff1c8ce2bf9e852ff8cf3f07a))
* voice chats v2 ([#414](https://github.com/stoatchat/stoatchat/issues/414)) ([d567155](https://github.com/stoatchat/stoatchat/commit/d567155f124e4da74115b1a8f810062f7c6559d9))


### Bug Fixes

* add license to revolt-parser ([5335124](https://github.com/stoatchat/stoatchat/commit/53351243064cac8d499dd74284be73928fa78a43))
* allow for disabling default features ([65fbd36](https://github.com/stoatchat/stoatchat/commit/65fbd3662462aed1333b79e59155fa6377e83fcc))
* apple music to use original url instead of metadata url ([bfe4018](https://github.com/stoatchat/stoatchat/commit/bfe4018e436a4075bae780dd4d35a9b58315e12f))
* apply uname fix to january and autumn ([8f9015a](https://github.com/stoatchat/stoatchat/commit/8f9015a6ff181d208d9269ab8691bd417d39811a))
* **ci:** publish images under stoatchat and remove docker hub ([d65c1a1](https://github.com/stoatchat/stoatchat/commit/d65c1a1ab3bdc7e5684b03f280af77d881661a3d))
* correct miniz_oxide in lockfile ([#478](https://github.com/stoatchat/stoatchat/issues/478)) ([5d27a91](https://github.com/stoatchat/stoatchat/commit/5d27a91e901dd2ea3e860aeaed8468db6c5f3214))
* correct shebang for try-tag-and-release ([050ba16](https://github.com/stoatchat/stoatchat/commit/050ba16d4adad5d0fb247867aa3e94e3d42bd12d))
* correct string_cache in lockfile ([#479](https://github.com/stoatchat/stoatchat/issues/479)) ([0b178fc](https://github.com/stoatchat/stoatchat/commit/0b178fc791583064bf9ca94b1d39b42d021e1d79))
* don't remove timeouts when a member leaves a server ([#409](https://github.com/stoatchat/stoatchat/issues/409)) ([e635bc2](https://github.com/stoatchat/stoatchat/commit/e635bc23ec857d648d5705e1a3875d7bc3402b0d))
* don't update the same field while trying to remove it ([f4ee35f](https://github.com/stoatchat/stoatchat/commit/f4ee35fb093ca49f0a64ff4b17fd61587df28145)), closes [#392](https://github.com/stoatchat/stoatchat/issues/392)
* github webhook incorrect payload and formatting ([#468](https://github.com/stoatchat/stoatchat/issues/468)) ([dc9c82a](https://github.com/stoatchat/stoatchat/commit/dc9c82aa4e9667ea6639256c65ac8de37a24d1f7))
* implement Serialize to ClientMessage ([dea0f67](https://github.com/stoatchat/stoatchat/commit/dea0f675dde7a63c7a59b38d469f878b7a8a3af4))
* newly created roles should be ranked the lowest ([947eb15](https://github.com/stoatchat/stoatchat/commit/947eb15771ed6785b3dcd16c354c03ded5e4cbe0))
* permit empty `remove` array in edit requests ([6ad3da5](https://github.com/stoatchat/stoatchat/commit/6ad3da5f35f989a2e7d8e29718b98374248e76af))
* preserve order of replies in message ([#447](https://github.com/stoatchat/stoatchat/issues/447)) ([657a3f0](https://github.com/stoatchat/stoatchat/commit/657a3f08e5d652814bbf0647e089ed9ebb139bbf))
* prevent timing out members which have TimeoutMembers permission ([e36fc97](https://github.com/stoatchat/stoatchat/commit/e36fc9738bac0de4f3fcbccba521f1e3754f7ae7))
* relax settings name regex ([3a34159](https://github.com/stoatchat/stoatchat/commit/3a3415915f0d0fdce1499d47a2b7fa097f5946ea))
* remove authentication tag bytes from attachment download ([32e6600](https://github.com/stoatchat/stoatchat/commit/32e6600272b885c595c094f0bc69459250220dcb))
* rename openapi operation ids ([6048587](https://github.com/stoatchat/stoatchat/commit/6048587d348fbca0dc3a9b47690c56df8fece576)), closes [#406](https://github.com/stoatchat/stoatchat/issues/406)
* respond with 201 if no body in requests ([#465](https://github.com/stoatchat/stoatchat/issues/465)) ([24fedf8](https://github.com/stoatchat/stoatchat/commit/24fedf8c4d9cd3160bdec97aa451520f8beaa739))
* swap to using reqwest for query building ([38dd4d1](https://github.com/stoatchat/stoatchat/commit/38dd4d10797b3e6e397fc219e818f379bdff19f2))
* use `trust_cloudflare` config value instead of env var ([cc7a796](https://github.com/stoatchat/stoatchat/commit/cc7a7962a882e1627fcd0bc75858a017415e8cfc))
* use our own result types instead of tenors types ([a92152d](https://github.com/stoatchat/stoatchat/commit/a92152d86da136997817e797c7af8e38731cdde8))

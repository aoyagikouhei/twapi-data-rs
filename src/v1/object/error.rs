use serde::{Deserialize, Serialize};

pub enum TwError {
    InvalidCoordinates,
    NoDataAvailableForTheGivenCoordinateAndRadius,
    NoDataAvailableForSpecifiedId,
    YouMustProvideValidCoordinatesIpAddressQueryOrAttributes { status_code: u16 },
    NoLocationAssociatedWithTheSpecifiedIpAddress,
    NoUserMatchesForSpecifiedTerms,
    QueryParametersAreMissing,
    CouldNotAuthenticateYou,
    SorryThatPageDoesNotExist,
    YouCannotReportYourselfForSpam,
    ParameterIsMissing { message: String },
    AttachmentUrlParameterIsInvalid,
    UserNotFound,
    UserHasBeenSuspended,
    YourAccountIsSuspendedAndIsNotPermittedToAccessThisFeature,
    SomeActionsOnThisUsersTweetHaveBeenDisabledByTwitterTheTwitterRestApiV1IsNoLongerActive,
    ClientIsNotPermittedToPerformThisAction,
    RateLimitExceeded,
    InvalidOrExpiredToken,
    SslIsRequired,
    ThisAppIsNotAllowedToAccessOrDeleteYourDirectMessages,
    UnableToVerifyYourCredentials,
    TheSpecifiedUserIsNotFoundInThisList,
    TheUserYouAreTryingToRemoveFromThisListIsNotAMember,
    AccountUpdateFailed { message: String },
    OverCapacity,
    InternalError,
    CouldNotAuthenticateYou135,
    YouHaveAlreadyFavoritedThisStatus,
    NoStatusFoundWithThatId,
    YouCannotSendMessagesToUsersWhoAreNotFollowingYou,
    ThereWasAnErrorSendingYourMessage { message: String },
    YouveAlreadyRequestedToFollowTheUser,
    YouAreUnableToFollowMorePeopleAtThisTime,
    CouldNotDetermineSourceUser,
    SorryYouAreNotAuthorizedToSeeThisStatus,
    UserIsOverDailyStatusUpdateLimit,
    TweetNeedsToBeABitShorter,
    StatusIsADuplicate,
    MissingOrInvalidUrlParameter,
    DeviceErrror { message: String },
    YouAreOverTheLimitForSpamReports,
    OwnerMustAllowDmsFromAnyone,
    BadAuthenticationData,
    YourCredentialsDoNotAllowAccessToThisResource,
    ThisRequestLooksLikeItMightBeAutomatedToProtectOurUsersFromSpamAndOtherMaliciousActivityWeCanTCompleteThisActionRightNow,
    ThisEndpointHasBeenRetiredAndShouldNotBeUsed,
    YourAppCannotPerformWriteActions,
    YouCanTMuteYourself,
    YouAreNotMutingTheSpecifiedUser,
    AnimatedGiFsAreNotAllowedWhenUploadingMultipleImages,
    TheValidationOfMediaIdsFailed,
    AMediaIdWasNotFound,
    VariableReturnedText { status_code: u16 },
    YouHaveAlreadyRetweetedThisTweet,
    YouCannotSendMessagesToThisUser,
    TheTextOfYourDirectMessageIsOverTheMaxCharacterLimit,
    SubscriptionAlreadyExists,
    YouAttemptedToReplyToATweetThatIsDeletedOrNotVisibleToYou,
    TheTweetExceedsTheNumberOfAllowedAttachmentTypes,
    CookiesAreRequiredToAccessTwitter,
    TheGivenUrlIsInvalid,
    CallbackUrlNotApprovedForThisClientAppApprovedCallbackUrLsCanBeAdjustedInYourAppsSettings,
    InvalidSuspendedApp,
    DesktopApplicationsOnlySupportTheOauthCallbackValueOob,
    ThisTweetIsNoLongerAvailable,
    ThisTweetIsNoLongerAvailableBecauseItViolatedTheTwitterRules,
    SomeActionsOnThisUsersTweetHaveBeenDisabledByTwitter,
    InvalidConversationControlParameterForThisEndpointYouNeedToChangeTheEndpointToASupportedEndpoint,
    TheOriginalTweetAuthorRestrictedWhoCanReplyToThisTweet,
}

impl From<TwError> for TwErrorItemWithStatusCode {
    fn from(from: TwError) -> TwErrorItemWithStatusCode {
        match from {
            TwError::InvalidCoordinates => TwErrorItemWithStatusCode::new(3, "Invalid coordinates.", 400),
            TwError::NoDataAvailableForTheGivenCoordinateAndRadius => TwErrorItemWithStatusCode::new(7, "No data available for the given coordinate and radius.", 400),
            TwError::NoDataAvailableForSpecifiedId => TwErrorItemWithStatusCode::new(8, "No data available for specified ID.", 400),
            TwError::YouMustProvideValidCoordinatesIpAddressQueryOrAttributes{status_code} => TwErrorItemWithStatusCode::new(12, "You must provide valid coordinates, IP address, query, or attributes.", status_code),
            TwError::NoLocationAssociatedWithTheSpecifiedIpAddress => TwErrorItemWithStatusCode::new(13, "No location associated with the specified IP address.", 404),
            TwError::NoUserMatchesForSpecifiedTerms => TwErrorItemWithStatusCode::new(17, "No user matches for specified terms.", 404),
            TwError::QueryParametersAreMissing => TwErrorItemWithStatusCode::new(25, "Query parameters are missing.", 404),
            TwError::CouldNotAuthenticateYou => TwErrorItemWithStatusCode::new(32, "Could not authenticate you.", 401),
            TwError::SorryThatPageDoesNotExist => TwErrorItemWithStatusCode::new(34, "Sorry, that page does not exist.", 404),
            TwError::YouCannotReportYourselfForSpam => TwErrorItemWithStatusCode::new(36, "You cannot report yourself for spam.", 403),
            TwError::ParameterIsMissing{message} => TwErrorItemWithStatusCode::new(38, &format!("{} parameter is missing.", message), 403),
            TwError::AttachmentUrlParameterIsInvalid => TwErrorItemWithStatusCode::new(44, "attachment_url parameter is invalid.", 400),
            TwError::UserNotFound => TwErrorItemWithStatusCode::new(50, "User not found.", 404),
            TwError::UserHasBeenSuspended => TwErrorItemWithStatusCode::new(63, "User has been suspended.", 403),
            TwError::YourAccountIsSuspendedAndIsNotPermittedToAccessThisFeature => TwErrorItemWithStatusCode::new(64, "Your account is suspended and is not permitted to access this feature.", 403),
            TwError::SomeActionsOnThisUsersTweetHaveBeenDisabledByTwitterTheTwitterRestApiV1IsNoLongerActive => TwErrorItemWithStatusCode::new(68, "Some actions on this user's Tweet have been disabled by Twitter. The Twitter REST API v1 is no longer active.", 410),
            TwError::ClientIsNotPermittedToPerformThisAction => TwErrorItemWithStatusCode::new(87, "Client is not permitted to perform this action.", 403),
            TwError::RateLimitExceeded => TwErrorItemWithStatusCode::new(88, "Rate limit exceeded.", 429),
            TwError::InvalidOrExpiredToken => TwErrorItemWithStatusCode::new(89, "Invalid or expired token.", 403),
            TwError::SslIsRequired => TwErrorItemWithStatusCode::new(92, "SSL is required.", 403),
            TwError::ThisAppIsNotAllowedToAccessOrDeleteYourDirectMessages => TwErrorItemWithStatusCode::new(93, "This App is not allowed to access or delete your Direct Messages.", 403),
            TwError::UnableToVerifyYourCredentials => TwErrorItemWithStatusCode::new(99, "Unable to verify your credentials.", 403),
            TwError::TheSpecifiedUserIsNotFoundInThisList => TwErrorItemWithStatusCode::new(109, "The specified user is not found in this list.", 404),
            TwError::TheUserYouAreTryingToRemoveFromThisListIsNotAMember => TwErrorItemWithStatusCode::new(110, "The user you are trying to remove from this list is not a member.", 400),
            TwError::AccountUpdateFailed{message} => TwErrorItemWithStatusCode::new(120, &format!("Account update failed: {}", message), 403),
            TwError::OverCapacity => TwErrorItemWithStatusCode::new(130, "Over capacity.", 503),
            TwError::InternalError => TwErrorItemWithStatusCode::new(131, "Internal error.", 500),
            TwError::CouldNotAuthenticateYou135 => TwErrorItemWithStatusCode::new(135, "Could not authenticate you.", 401),
            TwError::YouHaveAlreadyFavoritedThisStatus => TwErrorItemWithStatusCode::new(139, "You have already favorited this status.", 403),
            TwError::NoStatusFoundWithThatId => TwErrorItemWithStatusCode::new(144, "No status found with that ID.", 404),
            TwError::YouCannotSendMessagesToUsersWhoAreNotFollowingYou => TwErrorItemWithStatusCode::new(150, "You cannot send messages to users who are not following you.", 403),
            TwError::ThereWasAnErrorSendingYourMessage{message} => TwErrorItemWithStatusCode::new(151, &format!("There was an error sending your message: {}.", message), 403),
            TwError::YouveAlreadyRequestedToFollowTheUser => TwErrorItemWithStatusCode::new(160, "You've already requested to follow the user.", 403),
            TwError::YouAreUnableToFollowMorePeopleAtThisTime => TwErrorItemWithStatusCode::new(161, "You are unable to follow more people at this time.", 403),
            TwError::CouldNotDetermineSourceUser => TwErrorItemWithStatusCode::new(163, "Could not determine source user.", 403),
            TwError::SorryYouAreNotAuthorizedToSeeThisStatus => TwErrorItemWithStatusCode::new(179, "Sorry, you are not authorized to see this status.", 403),
            TwError::UserIsOverDailyStatusUpdateLimit => TwErrorItemWithStatusCode::new(185, "User is over daily status update limit.", 403),
            TwError::TweetNeedsToBeABitShorter => TwErrorItemWithStatusCode::new(186, "Tweet needs to be a bit shorter.", 403),
            TwError::StatusIsADuplicate => TwErrorItemWithStatusCode::new(187, "Status is a duplicate.", 403),
            TwError::MissingOrInvalidUrlParameter => TwErrorItemWithStatusCode::new(195, "Missing or invalid url parameter.", 403),
            TwError::DeviceErrror{message} => TwErrorItemWithStatusCode::new(203, &format!("Device errror: {}", message), 403),
            TwError::YouAreOverTheLimitForSpamReports => TwErrorItemWithStatusCode::new(205, "You are over the limit for spam reports.", 403),
            TwError::OwnerMustAllowDmsFromAnyone => TwErrorItemWithStatusCode::new(214, "Owner must allow dms from anyone.", 403),
            TwError::BadAuthenticationData => TwErrorItemWithStatusCode::new(215, "Bad authentication data.", 400),
            TwError::YourCredentialsDoNotAllowAccessToThisResource => TwErrorItemWithStatusCode::new(220, "Your credentials do not allow access to this resource.", 403),
            TwError::ThisRequestLooksLikeItMightBeAutomatedToProtectOurUsersFromSpamAndOtherMaliciousActivityWeCanTCompleteThisActionRightNow => TwErrorItemWithStatusCode::new(226, "This request looks like it might be automated. To protect our users from spam and other malicious activity, we can’t complete this action right now.", 403),
            TwError::ThisEndpointHasBeenRetiredAndShouldNotBeUsed => TwErrorItemWithStatusCode::new(251, "This endpoint has been retired and should not be used.", 410),
            TwError::YourAppCannotPerformWriteActions => TwErrorItemWithStatusCode::new(261, "Your App cannot perform write actions.", 403),
            TwError::YouCanTMuteYourself => TwErrorItemWithStatusCode::new(271, "You can’t mute yourself.", 403),
            TwError::YouAreNotMutingTheSpecifiedUser => TwErrorItemWithStatusCode::new(272, "You are not muting the specified user.", 403),
            TwError::AnimatedGiFsAreNotAllowedWhenUploadingMultipleImages => TwErrorItemWithStatusCode::new(323, "Animated GIFs are not allowed when uploading multiple images.", 400),
            TwError::TheValidationOfMediaIdsFailed => TwErrorItemWithStatusCode::new(324, "The validation of media ids failed.", 400),
            TwError::AMediaIdWasNotFound => TwErrorItemWithStatusCode::new(325, "A media id was not found.", 400),
            TwError::VariableReturnedText{status_code} => TwErrorItemWithStatusCode::new(326, "Variable returned text", status_code),
            TwError::YouHaveAlreadyRetweetedThisTweet => TwErrorItemWithStatusCode::new(327, "You have already retweeted this Tweet.", 403),
            TwError::YouCannotSendMessagesToThisUser => TwErrorItemWithStatusCode::new(349, "You cannot send messages to this user.", 403),
            TwError::TheTextOfYourDirectMessageIsOverTheMaxCharacterLimit => TwErrorItemWithStatusCode::new(354, "The text of your direct message is over the max character limit.", 403),
            TwError::SubscriptionAlreadyExists => TwErrorItemWithStatusCode::new(355, "Subscription already exists.", 409),
            TwError::YouAttemptedToReplyToATweetThatIsDeletedOrNotVisibleToYou => TwErrorItemWithStatusCode::new(385, "You attempted to reply to a Tweet that is deleted or not visible to you.", 403),
            TwError::TheTweetExceedsTheNumberOfAllowedAttachmentTypes => TwErrorItemWithStatusCode::new(386, "The Tweet exceeds the number of allowed attachment types.", 403),
            TwError::CookiesAreRequiredToAccessTwitter => TwErrorItemWithStatusCode::new(404, "Cookies are required to access Twitter", 422),
            TwError::TheGivenUrlIsInvalid => TwErrorItemWithStatusCode::new(407, "The given URL is invalid.", 400),
            TwError::CallbackUrlNotApprovedForThisClientAppApprovedCallbackUrLsCanBeAdjustedInYourAppsSettings => TwErrorItemWithStatusCode::new(415, "Callback URL not approved for this client App. Approved callback URLs can be adjusted in your App's settings.", 403),
            TwError::InvalidSuspendedApp => TwErrorItemWithStatusCode::new(416, "Invalid / suspended App.", 401),
            TwError::DesktopApplicationsOnlySupportTheOauthCallbackValueOob => TwErrorItemWithStatusCode::new(417, "Desktop applications only support the oauth_callback value 'oob'.", 401),
            TwError::ThisTweetIsNoLongerAvailable => TwErrorItemWithStatusCode::new(421, "This Tweet is no longer available.", 404),
            TwError::ThisTweetIsNoLongerAvailableBecauseItViolatedTheTwitterRules => TwErrorItemWithStatusCode::new(422, "This Tweet is no longer available because it violated the Twitter Rules.", 404),
            TwError::SomeActionsOnThisUsersTweetHaveBeenDisabledByTwitter => TwErrorItemWithStatusCode::new(425, "Some actions on this user's Tweet have been disabled by Twitter.", 403),
            TwError::InvalidConversationControlParameterForThisEndpointYouNeedToChangeTheEndpointToASupportedEndpoint => TwErrorItemWithStatusCode::new(431, "Invalid conversation control parameter for this endpoint. You need to change the endpoint to a supported endpoint.", 403),
            TwError::TheOriginalTweetAuthorRestrictedWhoCanReplyToThisTweet => TwErrorItemWithStatusCode::new(433, "The original Tweet author restricted who can reply to this Tweet.", 403),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TwErrorItemWithStatusCode {
    error: TwErrorItem,
    status_code: u16,
}

impl TwErrorItemWithStatusCode {
    pub fn new(code: u16, message: &str, status_code: u16) -> Self {
        Self {
            error: TwErrorItem {
                code: code,
                message: message.to_owned(),
            },
            status_code: status_code,
        }
    }

    pub fn make_errors(&self) -> TwErrors {
        TwErrors {
            errors: vec![self.error.clone()],
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwErrors {
    pub errors: Vec<TwErrorItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwErrorItem {
    pub code: u16,
    pub message: String,
}

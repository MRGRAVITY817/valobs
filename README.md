# VALOBS

A collection of commonly used **VAL**ue **OB**ject**S** in enterprise applications

## What are _value objects?_

In Domain-Driven Design (DDD), a Value Object is one of the building blocks used to model the domain. Value Objects are objects whose equality is determined by the value of their attributes rather than by their identity. In other words, two Value Objects are considered equal if they have the same set of attribute values, even if they are distinct instances.

Here are some key characteristics and considerations for Value Objects in DDD:

- Immutability: Value Objects are typically immutable, meaning their state cannot be changed once they are created. This immutability ensures that the value of a Value Object remains constant throughout its lifetime.

- Equality: Equality of Value Objects is based on the equality of their attributes. If two Value Objects have the same attribute values, they are considered equal, regardless of their individual identity.

- Identity Ignored: Unlike entities, which have a distinct identity, Value Objects do not have a conceptual identity. They are defined only by their attributes. This means that if you create a new Value Object with the same attribute values as an existing one, they are considered equal.

- Side-Effect-Free: Value Objects should not have any side effects. Operations on Value Objects should not change the state of the system or other objects.

- Examples: Common examples of Value Objects include simple types such as strings, numbers, and dates, as well as more complex structures like Money, Address, or PhoneNumber. These objects are characterized by the fact that their identity is determined by their attribute values.

Using Value Objects in your domain model helps to clarify the semantics of the model and can lead to more expressive and intention-revealing code. They are particularly useful for modeling concepts in your domain that are defined by their attributes rather than their identity.

## Collated List

### **Numeric Values**

- [ ] Integer
- [ ] Decimal
- [ ] Percentage
- [ ] Ratio
- [ ] CurrencyAmount
- [ ] MeasurementUnit
- [ ] Fraction

### **Temporal Values**

- [x] Date
- [x] Time
- [x] DateTime
- [x] Duration
- [ ] TimeInterval
- [ ] Timezone

### **Geographical Values**

- [ ] Latitude
- [ ] Longitude
- [ ] GeoLocation
- [ ] Address
- [ ] Country
- [ ] Continent

### **Communication**

- [ ] EmailAddress
- [ ] PhoneNumber
- [ ] URL
- [ ] SocialMediaHandle
- [ ] PostalAddress

### **Identification**

- [ ] UUID
- [ ] GUID
- [ ] SerialNumber
- [ ] NationalID
- [ ] PassportNumber
- [ ] EmployeeID

### **Measurement**

- [ ] Length
- [ ] Weight
- [ ] Volume
- [ ] Area
- [ ] Temperature
- [ ] Speed

### **Textual Values**

- [ ] Name
- [ ] Description
- [ ] Title
- [ ] Comment
- [ ] TextContent
- [ ] Tag

### **Financial Values**

- [x] Money
- [ ] CurrencyExchangeRate
- [ ] CreditCard
- [ ] PaymentTransaction
- [ ] Invoice
- [ ] FinancialAmount

### **Code and Configuration**

- [ ] StatusCode
- [ ] ConfigurationParameter
- [ ] FeatureToggle
- [ ] CodeSnippet
- [ ] VersionNumber
- [ ] APIKey

### **Security**

- [ ] Password
- [ ] HashValue
- [ ] EncryptionKey
- [ ] DigitalSignature
- [ ] OAuthToken
- [ ] APIAccessToken

### **Media and Files**

- [ ] File
- [ ] Image
- [ ] Video
- [ ] Audio
- [ ] Document
- [ ] FileFormat

### **Social and Networking**

- [ ] ProfileID
- [ ] ConnectionRequest
- [ ] RelationshipStatus
- [ ] Notification
- [ ] GroupID
- [ ] ChatMessage

### **Health and Fitness**

- [ ] HeartRate
- [ ] BloodPressure
- [ ] CaloricIntake
- [ ] StepsCount
- [ ] BMI
- [ ] SleepDuration

### **Scientific Values**

- [ ] ScientificNotation
- [ ] AtomicNumber
- [ ] MolecularWeight
- [ ] pHValue
- [ ] Gravity

### **Education**

- [ ] StudentID
- [ ] CourseCode
- [ ] Grade
- [ ] EducationalInstitution
- [ ] Transcript
- [ ] ExamScore

### **Transportation**

- [ ] VehicleIdentificationNumber (VIN)
- [ ] LicensePlate
- [ ] Route
- [ ] FlightNumber
- [ ] TicketNumber
- [ ] FuelConsumption

### **Environmental Values**

- [ ] CarbonFootprint
- [ ] PollutionIndex
- [ ] RenewableEnergyProduction
- [ ] EnvironmentalImpact

### **Legal and Compliance**

- [ ] ContractID
- [ ] ComplianceStatus
- [ ] LegalCaseNumber
- [ ] RegulatoryApproval
- [ ] LicenseNumber

### **Custom Codes and Identifiers**

- [ ] CustomCode
- [ ] UniqueIdentifier
- [ ] BusinessID
- [ ] EntityCode
- [ ] ReferenceNumber

### **Miscellaneous**

- [ ] Barcode
- [ ] Color
- [ ] ISBN
- [ ] DomainName
- [ ] Language
- [ ] DeviceID

### **Numeric Units**

- [ ] Speed (e.g., meters per second)
- [ ] Acceleration
- [ ] Voltage
- [ ] Current
- [ ] Resistance
- [ ] Frequency

### **Temporal Intervals**

- [ ] FiscalYear
- [ ] AcademicTerm
- [ ] BookingPeriod
- [ ] ProjectTimeline
- [ ] Season
- [ ] EpochTime

### **Geographical Regions**

- [ ] State
- [ ] Province
- [ ] City
- [ ] District
- [ ] Neighborhood
- [ ] Landmark

### **Communication Channels**

- [ ] CommunicationProtocol
- [ ] MessagingQueue
- [ ] ChatRoom
- [ ] VoIPCallID
- [ ] BroadcastChannel

### **Identification Verification**

- [ ] BiometricData
- [ ] FacialRecognitionData
- [ ] VoicePrint
- [ ] Fingerprint
- [ ] RetinaScan

### **Measurement Accuracy**

- [ ] PrecisionLevel
- [ ] Tolerance
- [ ] MarginOfError
- [ ] Sensitivity
- [ ] CalibrationValue

### **Textual Styles**

- [ ] FontStyle
- [ ] TextAlignment
- [ ] MarkupLanguage
- [ ] WritingStyle
- [ ] LanguageVariant

### **Financial Transactions**

- [ ] TransactionID
- [ ] BankingTransactionType
- [ ] InvestmentPortfolio
- [ ] TaxID
- [ ] PayoutAmount

### **Code and Configuration Settings**

- [ ] ConfigurationKey
- [ ] EnvironmentVariable
- [ ] FeatureFlagStatus
- [ ] DebugMode
- [ ] LogLevel

### **Security Access**

- [ ] AccessControlList (ACL)
- [ ] PermissionSet
- [ ] SecurityClearanceLevel
- [ ] BiometricAccessLevel
- [ ] SecureToken

### **Media Metadata**

- [ ] MetadataTag
- [ ] ExifData
- [ ] CopyrightInformation
- [ ] DigitalAssetID
- [ ] MediaResolution

### **Social Interactions**

- [ ] SocialEventID
- [ ] AttendeeList
- [ ] SocialInteractionType
- [ ] SocialGroupID
- [ ] EventLocation

### **Health Metrics**

- [ ] CholesterolLevel
- [ ] BloodSugarLevel
- [ ] FitnessGoal
- [ ] HealthInsurancePolicy
- [ ] MedicalHistory

### **Scientific Constants**

- [ ] PlanckConstant
- [ ] AvogadroNumber
- [ ] GravitationalConstant
- [ ] SpeedOfLight
- [ ] BoltzmannConstant

### **Educational Achievements**

- [ ] DiplomaID
- [ ] Certification
- [ ] DegreeType
- [ ] AcademicAward
- [ ] GraduationStatus

### **Transportation Routes**

- [ ] TrafficCondition
- [ ] NavigationRoute
- [ ] RouteOptimization
- [ ] TransportationMode
- [ ] ParkingSpotID

### **Environmental Conservation**

- [ ] EcologicalFootprint
- [ ] BiodiversityIndex
- [ ] EnvironmentalPolicy
- [ ] ConservationAreaID

### **Legal Documents**

- [ ] LegalDocumentID
- [ ] LegalCaseStatus
- [ ] CourtOrder
- [ ] LegalRepresentation
- [ ] NotarizationRecord

### **Custom Identifiers**

- [ ] CustomIdentifier
- [ ] EntityAlias
- [ ] ReferenceCode
- [ ] ClientID
- [ ] VendorCode

### **Miscellaneous Concepts**

- [ ] AugmentedRealityMarker
- [ ] VirtualRealityEnvironment
- [ ] SubscriptionPlan
- [ ] TradeOrderID
- [ ] UniqueDeviceIdentifier (UDID)
